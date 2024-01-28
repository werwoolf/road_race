use rand::{Rng, thread_rng};
use rusty_engine::prelude::*;

pub struct GameState {
    pub(crate) health_amount: u8,
    pub(crate) lost: bool,
}

impl Resource for GameState {}

pub struct GameService<'a> {
    game: &'a mut Game<GameState>,
}

impl<'a> GameService<'a> {
    const OBSTACLES_SET: [SpritePreset; 15] = [
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingBarrierRed,
        SpritePreset::RacingBarrierWhite,
        SpritePreset::RacingConeStraight,
        SpritePreset::RollingBallBlue,
        SpritePreset::RollingBallBlueAlt,
        SpritePreset::RollingBallRed,
        SpritePreset::RollingBallRedAlt,
        SpritePreset::RollingBlockCorner,
        SpritePreset::RollingBlockNarrow,
        SpritePreset::RollingBlockSmall,
        SpritePreset::RollingBlockSquare,
        SpritePreset::RollingHoleEnd,
        SpritePreset::RollingHoleStart
    ];
}

impl<'a> GameService<'a> {
    pub(crate) fn new(game: &'a mut Game<GameState>) -> GameService {
        Self { game }
    }

    pub fn create_player(&mut self) {
        let player = self.game.add_sprite("player_1", SpritePreset::RacingCarBlue);
        player.translation.x = -500.0;
        player.layer = 10.0;
        player.collision = true;
    }

    pub fn set_health_text(&mut self, amount: u32) {
        let health_message = self.game.add_text("health_message", format!("Health: {amount}"));
        health_message.translation = Vec2::new(550.0, 320.0);
    }

    pub fn create_road_lines(&mut self, count: i32) {
        for i in 0..count {
            let road_line = self.game.add_sprite(format!("road_line{}", i), SpritePreset::RacingBarrierWhite);
            road_line.scale = 0.1;
            road_line.translation.x = -600.0 + 150.0 * i as f32;
        }
    }

    pub fn create_obstacles(&mut self, count: i32) {
        for i in 0..count {
            let rnd_sprite_index = thread_rng().gen_range(0..GameService::OBSTACLES_SET.len());
            let obstacle_obj = self.game.add_sprite(format!("obstacle_{i}"), GameService::OBSTACLES_SET[rnd_sprite_index]);
            obstacle_obj.layer = 5.0;
            obstacle_obj.collision = true;
            obstacle_obj.translation.x = thread_rng().gen_range(800.0..1600.0);
            obstacle_obj.translation.y = thread_rng().gen_range(-300.0..300.0);
        }
    }
}
