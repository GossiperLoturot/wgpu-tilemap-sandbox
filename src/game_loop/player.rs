//! プレイヤーシステムの機能に関するモジュール

use glam::*;

use crate::game_loop::entity;

/// プレイヤーシステムの機能
#[derive(Default)]
pub struct PlayerSystem {
    player_id: Option<usize>,
}

impl PlayerSystem {
    /// 通常時の移動速度
    const DEFAULT_SPEED: f32 = 2.0;

    /// スプリント時の移動速度
    const SPRINT_SPEED: f32 = 4.0;

    /// 新しいプレイヤーを作成し、そのエンティティの参照を返す。
    pub fn spawn_player<'a>(
        &mut self,
        entity_system: &'a mut entity::EntitySystem,
    ) -> Option<&'a entity::Entity> {
        if self.player_id.is_none() {
            let id = entity_system.insert(entity::Entity::new(
                vec2(0.0, 0.0),
                entity::EntityKind::Player,
            ));
            self.player_id = Some(id);
            self.get_player(entity_system)
        } else {
            None
        }
    }

    /// プレイヤーを削除し、そのエンティティを返す。
    pub fn despawn_player(
        &mut self,
        entity_system: &mut entity::EntitySystem,
    ) -> Option<entity::Entity> {
        self.player_id.and_then(|id| entity_system.remove(id))
    }

    /// プレイヤーの参照を返す。
    pub fn get_player<'a>(
        &self,
        entity_system: &'a entity::EntitySystem,
    ) -> Option<&'a entity::Entity> {
        self.player_id.and_then(|id| entity_system.get(id))
    }

    /// プレイヤーの可変参照を返す。
    pub fn get_player_mut<'a>(
        &self,
        entity_system: &'a mut entity::EntitySystem,
    ) -> Option<&'a mut entity::Entity> {
        self.player_id.and_then(|id| entity_system.get_mut(id))
    }

    /// ゲームサイクルにおけるプレイヤーの振る舞いを実行する。
    ///
    /// 振る舞いは以下のとおりである。
    ///
    /// - プレイヤーはLShiftを押下時スプリントを行う。
    /// - プレイヤーはWSADで上下左右の移動を行う。
    pub fn update(
        &mut self,
        entity_system: &mut entity::EntitySystem,
        input: &winit_input_helper::WinitInputHelper,
        elased: std::time::Duration,
    ) {
        if let Some(id) = self.player_id {
            if let Some(mut player) = entity_system.remove(id) {
                let speed = if input.key_held(winit::event::VirtualKeyCode::LShift) {
                    Self::SPRINT_SPEED
                } else {
                    Self::DEFAULT_SPEED
                };

                if input.key_held(winit::event::VirtualKeyCode::W) {
                    player.position.y += speed * elased.as_secs_f32();
                }
                if input.key_held(winit::event::VirtualKeyCode::S) {
                    player.position.y -= speed * elased.as_secs_f32();
                }
                if input.key_held(winit::event::VirtualKeyCode::A) {
                    player.position.x -= speed * elased.as_secs_f32();
                }
                if input.key_held(winit::event::VirtualKeyCode::D) {
                    player.position.x += speed * elased.as_secs_f32();
                }

                entity_system.insert(player);
            }
        }
    }
}