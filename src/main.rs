mod assets;
mod game_loop;
mod renderer;

fn main() {
    let assets = assets::Assets::new("assets/assets.json");

    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    let window = winit::window::WindowBuilder::new()
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut game_loop = game_loop::GameLoop::new();
    let mut renderer = pollster::block_on(renderer::Renderer::new_async(&assets, &window));
    let mut input = winit_input_helper::WinitInputHelper::new();

    let mut instant = std::time::Instant::now();
    let mut read_back = None;

    use winit::event::Event;
    use winit::event::WindowEvent;
    event_loop
        .run(move |event, control_flow| {
            input.update(&event);

            match event {
                Event::AboutToWait => {
                    window.request_redraw();
                }
                Event::WindowEvent { window_id, event } if window_id == window.id() => {
                    match event {
                        WindowEvent::RedrawRequested => {
                            let elapsed =
                                std::mem::replace(&mut instant, std::time::Instant::now())
                                    .elapsed();

                            let cx = game_loop::InputContext {
                                assets: &assets,
                                input: &input,
                                read_back: &read_back,
                                elapsed: &elapsed,
                            };
                            game_loop.update(&cx);
                            let extract = game_loop.extract(&cx);

                            let cx = renderer::InputContext {
                                assets: &assets,
                                input: &input,
                                read_back: &read_back,
                                elapsed: &elapsed,
                            };
                            read_back = Some(renderer.draw(&cx, &extract));
                        }
                        WindowEvent::CloseRequested => {
                            control_flow.exit();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .unwrap();
}
