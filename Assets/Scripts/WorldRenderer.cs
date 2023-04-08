using System.Collections.Generic;
using System.Threading;
using UnityEngine;
using UnityEngine.AddressableAssets;

[RequireComponent(typeof(Camera))]
public class WorldRenderer : MonoBehaviour
{
    private const string PRELOAD_LABELS = "preload";
    private const string FALLBACK_RESOURCE_NAME = "Fallback";
    private const int SLEEP_MILLISECONDS = 32;

    private Camera camera;
    private Dictionary<string, GameObject> prefabs;
    private Dictionary<Vector3Int, GameObject> tileInstances;
    private Dictionary<string, GameObject> entityInstances;

    private BoundsInt? bounds;
    private bool subThreadEnabled;
    private Thread subThread;

    private void Start()
    {
        camera = GetComponent<Camera>();
        prefabs = new();
        tileInstances = new();
        entityInstances = new();

        var resourceLocationsHandle = Addressables.LoadResourceLocationsAsync(PRELOAD_LABELS);
        foreach (var resourceLocation in resourceLocationsHandle.WaitForCompletion())
        {
            var prefabHandle = Addressables.LoadAssetAsync<GameObject>(resourceLocation);
            prefabs.Add(resourceLocation.PrimaryKey, prefabHandle.WaitForCompletion());
            Addressables.Release(prefabHandle);
        }
        Addressables.Release(resourceLocationsHandle);

        subThreadEnabled = true;
        subThread = new Thread(new ThreadStart(UpdateInSubThread));
        subThread.Start();
    }

    private void UpdateInSubThread()
    {
        while (subThreadEnabled)
        {
            WorldService.mutex.WaitOne();

            WorldService.update.DispatchUpdateEvent();

            if (this.bounds is BoundsInt bounds)
            {
                WorldService.generation.SetUpdateBounds(bounds);
                WorldService.tile.SetUpdateBounds(bounds);
                WorldService.entity.SetUpdateBounds(bounds);
            }

            WorldService.mutex.ReleaseMutex();

            Thread.Sleep(SLEEP_MILLISECONDS);
        }
    }

    private void Update()
    {
        var widthExtent = Mathf.CeilToInt(camera.orthographicSize * camera.aspect);
        var heightExtent = Mathf.CeilToInt(camera.orthographicSize);
        var layerExtent = Mathf.CeilToInt(transform.position.z);

        var center = Vector3Int.RoundToInt(new Vector3(transform.position.x, transform.position.y));
        var extent = new Vector3Int(widthExtent, heightExtent, layerExtent);

        var bounds = new BoundsInt();
        bounds.SetMinMax(center - extent, center + extent);
        this.bounds = bounds;

        WorldService.mutex.WaitOne();

        foreach (var cmd in WorldService.tile.GetUpdateCommands())
        {
            switch (cmd)
            {
                case TileService.AddTileCommand addCmd:
                    var instance = Instantiate(GetPrefab(addCmd.tile.resourceName), GetProjectionPos(addCmd.tile.pos), Quaternion.identity);
                    var customProperty = instance.AddComponent<CustomPropertyTile>();
                    customProperty.value = addCmd.tile.pos;
                    tileInstances.Add(addCmd.tile.pos, instance);
                    break;

                case TileService.RemoveTileCommand removeCmd:
                    Destroy(tileInstances[removeCmd.pos]);
                    tileInstances.Remove(removeCmd.pos);
                    break;
            }
        }

        foreach (var cmd in WorldService.entity.GetUpdateCommands())
        {
            switch (cmd)
            {
                case EntityService.AddEntityCommand addCmd:
                    var instance = Instantiate(GetPrefab(addCmd.entity.resourceName), GetProjectionPos(addCmd.entity.pos), Quaternion.identity);
                    var customProperty = instance.AddComponent<CustomPropertyEntity>();
                    customProperty.value = addCmd.entity.id;
                    entityInstances.Add(addCmd.entity.id, instance);
                    break;

                case EntityService.RemoveEntityCommand removeCmd:
                    Destroy(entityInstances[removeCmd.id]);
                    entityInstances.Remove(removeCmd.id);
                    break;
            }
        }

        WorldService.mutex.ReleaseMutex();
    }

    private GameObject GetPrefab(string resourceName)
    {
        if (prefabs.ContainsKey(resourceName))
        {
            return prefabs[resourceName];
        }
        else
        {
            return prefabs[FALLBACK_RESOURCE_NAME];
        }
    }

    private Vector3 GetProjectionPos(Vector3 pos)
    {
        return new Vector3(pos.x, pos.y + pos.z, -pos.z);
    }

    private void OnDestroy()
    {
        subThreadEnabled = false;
        subThread.Join();
    }
}
