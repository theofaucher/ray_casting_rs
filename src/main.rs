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

#[macroquad::main(window_conf)]
async fn main() {

    let map : map::Map = map::Map::new(MAP_SIZE_X, MAP_SIZE_Y, MAP);
    let mut player: player::Player = player::Player::new(300, 300, 5, 16);

    player.turn_right();
    player.turn_left();

    loop {
        clear_background(LIGHTGRAY);
        map.draw();
        get_user_input(&mut player);
        map.move_player(& player);

        next_frame().await
    }
}

fn get_user_input(player: &mut player::Player) {

    if is_key_down(KeyCode::Escape) {
        std::process::exit(0);
    } else if is_key_down(KeyCode::Right) {
        player.turn_right();
    } else if is_key_down(KeyCode::Left) {
        player.turn_left();
    } else if is_key_down(KeyCode::Up) {
        player.move_forward();
    } else if is_key_down(KeyCode::Down) {
        player.move_backward();
    }

}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "RayCasting".to_string(),
        window_width: 1024,
        window_height: 512,
        window_resizable: false,
        ..Default::default()
    }
}