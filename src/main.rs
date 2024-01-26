use rusty_engine::prelude::*;
use rand::prelude::*;

struct GameState {
    health_amount: u8,
    lost: bool,
}

impl Resource for GameState {}

const ROAD_SPEED: f32 = 400.0;

fn main() {
    let mut game = Game::new();

    let player = game.add_sprite("player_1", SpritePreset::RacingCarBlue);
    player.translation.x = -500.0;
    player.layer = 10.0;
    player.collision = true;

    let health_message = game.add_text("health_message", "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    for i in 0..10 {
        let road_line = game.add_sprite(format!("road_line{}", i), SpritePreset::RacingBarrierWhite);
        road_line.scale = 0.1;
        road_line.translation.x = -600.0 + 150.0 * i as f32;
    }

    let obstacles_preset = vec![
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingConeStraight,
        SpritePreset::RacingConeStraight,
    ];

    for (i, obstacle) in obstacles_preset.into_iter().enumerate() {
        let obstacle_obj = game.add_sprite(format!("obstacle_{i}"), obstacle);
        obstacle_obj.layer = 5.0;
        obstacle_obj.collision = true;
        obstacle_obj.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle_obj.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.lost {
        return;
    }
    const PLAYER_SPEED: f32 = 250.0;
    let mut direction: f32 = 0.0;

    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction = 1.0;
    }

    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction = -1.0;
    }

    let player = engine.sprites.get_mut("player_1").unwrap();
    player.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player.rotation = direction * 0.15;

    if player.translation.y < -360.0 || player.translation.y > 360.0 {
        game_state.health_amount = 0;
    }

    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("road_line") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;

            if sprite.translation.x < -650.0 {
                sprite.translation.x += 1500.0;
            }
        } else if sprite.label.starts_with("obstacle_") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;

            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }


    for collision in engine.collision_events.clone().drain(..) {
        if !collision.pair.one_starts_with("player_1") || collision.state.is_end() {
            continue;
        }

        let health_message = engine.texts.get_mut("health_message").unwrap();
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        }

        if game_state.health_amount == 0 {
            game_state.lost = true;
            let game_over = engine.add_text("game over", "Game Over");
            game_over.font_size = 128.0;
            engine.audio_manager.stop_music();
            engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
        }

    }
}