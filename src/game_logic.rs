use rand::{Rng, thread_rng};
use rusty_engine::audio::SfxPreset;
use rusty_engine::game::Engine;
use rusty_engine::keyboard::KeyCode;
use crate::game_service::{GameState};

const ROAD_SPEED: f32 = 400.0;

pub struct GameLogic<'a> {
    engine: &'a mut Engine,
    game_state: &'a mut GameState,
    direction: f32
}

impl<'a> GameLogic<'a>{
    const PLAYER_SPEED: f32 = 250.0;
}

impl<'a> GameLogic<'a> {
    pub fn new(engine: &'a mut Engine, game_state: &'a mut GameState) -> GameLogic<'a> {
        GameLogic {
            engine,
            game_state,
            direction: 0.0
        }
    }
    pub fn logic(&mut self) {
        if self.game_state.lost {
            return;
        }

        if self.engine.keyboard_state.pressed(KeyCode::Up) {
            self.direction = 1.0;
        }

        if self.engine.keyboard_state.pressed(KeyCode::Down) {
            self.direction = -1.0;
        }

        let player = self.engine.sprites.get_mut("player_1").unwrap();
        player.translation.y += self.direction * GameLogic::PLAYER_SPEED * self.engine.delta_f32;
        player.rotation = self.direction * 0.15;

        if player.translation.y < -360.0 || player.translation.y > 360.0 {
            self.game_state.health_amount = 0;
        }
    }

    pub fn move_sprites(&mut self){
        for sprite in self.engine.sprites.values_mut() {
            if sprite.label.starts_with("road_line") {
                sprite.translation.x -= ROAD_SPEED * self.engine.delta_f32;

                if sprite.translation.x < -650.0 {
                    sprite.translation.x += 1500.0;
                }
            } else if sprite.label.starts_with("obstacle_") {
                sprite.translation.x -= ROAD_SPEED * self.engine.delta_f32;

                if sprite.translation.x < -800.0 {
                    sprite.translation.x = thread_rng().gen_range(700.0..2000.0);
                    sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
                }
            }
        }
    }

    pub fn listen_collisions(&mut self){
        for collision in self.engine.collision_events.clone().drain(..) {
            if !collision.pair.one_starts_with("player_1") || collision.state.is_end() {
                continue;
            }

            if self.game_state.health_amount > 0 {
                let health_message = self.engine.texts.get_mut("health_message").unwrap();
                self.game_state.health_amount -= 1;
                health_message.value = format!("Health: {}", self.game_state.health_amount);
                self.engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
            }

            if self.game_state.health_amount == 0 {
                self.game_state.lost = true;
                let game_over = self.engine.add_text("game over", "Game Over");
                game_over.font_size = 128.0;
                self.engine.audio_manager.stop_music();
                self.engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
            }
        }
    }
}