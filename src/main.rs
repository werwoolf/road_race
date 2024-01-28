mod game_service;
mod game_logic;

use rusty_engine::prelude::*;
use rand::prelude::*;
use game_service::GameService;
use crate::game_service::GameState;
use crate::game_logic::GameLogic;

const ROAD_SPEED: f32 = 400.0;

fn main() {
    let mut game = Game::new();

    let mut game_service = GameService::new(&mut game);

    game_service.create_player();
    game_service.create_road_lines(10);
    game_service.create_obstacles(8);

    game_service.set_health_text(5);

    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 250,
        lost: false,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let mut logic = GameLogic::new(engine, game_state);
    logic.logic();
    logic.listen_collisions();
    logic.move_sprites();
}
