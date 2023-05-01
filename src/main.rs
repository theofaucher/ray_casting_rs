use macroquad::color::LIGHTGRAY;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::{clear_background, Conf, next_frame};

mod player;
mod map;

static MAP_SIZE_X: u8 = 8;
static MAP_SIZE_Y: u8 = 8;
static MAP: [bool; 64] =
    [true, true,  true,  true,  true,  true,  true,  true,
     true, false, false, false, false, false, false, true,
     true, false, false, false, false, false, false, true,
     true, true,  true,  true,  false, false, true,  true,
     true, false, false, false, false, false, false, true,
     true, false, false, true,  false, true,  false, true,
     true, false, false, true,  false, true,  false, true,
     true, true,  true,  true,  true,  true,  true,  true];

pub fn window_conf() -> Conf {
    Conf {
        window_title: "RayCasting".to_string(),
        window_width: 1024,
        window_height: 512,
        window_resizable: false,
        ..Default::default()
    }
}

fn get_user_input(player: &mut player::Player) {
    let player_size = player.get_size();

    if is_key_down(KeyCode::Right) {
        if (player.get_x_position() + (player_size as f32/2.) as u32) < 512 {
            player.update_x_position(8);
        } else {
            player.set_x_position(512 - (player_size as f32/2.) as u32);
        }
    } else if is_key_down(KeyCode::Left) {
        if (player.get_x_position() - 8) >= (player_size as f32/2.) as u32 {
            player.update_x_position(-8);
        } else {
            player.set_x_position((player_size as f32/2.) as u32);
        }
    } else if is_key_down(KeyCode::Up) {
        if (player.get_y_position() - 8) >= (player_size as f32/2.) as u32 {
            player.update_y_position(-8);
        } else {
            player.set_y_position((player_size as f32/2.) as u32);
        }
    } else if is_key_down(KeyCode::Down) {
        if (player.get_y_position() + (player_size as f32/2.) as u32) < 512 {
            player.update_y_position(8);
        } else {
            player.set_y_position(512 - (player_size as f32/2.) as u32);
        }
    }
}
#[macroquad::main(window_conf)]
async fn main() {

    let map : map::Map = map::Map::new(MAP_SIZE_X, MAP_SIZE_Y, MAP);
    let mut player : player::Player = player::Player::new(300, 300, 16);

    loop {
        clear_background(LIGHTGRAY);
        map.draw();
        get_user_input(&mut player);
        map.move_player(player);

        next_frame().await
    }
}