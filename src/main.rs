use macroquad::prelude::*;

static MAP_SIZE_X: u8 = 8;
static MAP_SIZE_Y: u8 = 8;
static MAP_SIZE: usize = (MAP_SIZE_X * MAP_SIZE_Y) as usize;
static MAP: [bool; 64] =
    [true, true,  true,  true,  true,  true,  true,  true,
     true, false, false, false, false, false, false, true,
     true, false, false, false, false, false, false, true,
     true, true,  true,  true,  false, false, true,  true,
     true, false, false, false, false, false, false, true,
     true, false, false, true,  false, true,  false, true,
     true, false, false, true,  false, true,  false, true,
     true, true,  true,  true,  true,  true,  true,  true];

fn window_conf() -> Conf {
    Conf {
        window_title: "RayCasting".to_string(),
        window_width: 1024,
        window_height: 512,
        window_resizable: false,
        ..Default::default()
    }
}
fn move_player(player_size : u32, pos_x : u32, pos_y : u32) {

    draw_rectangle(
        pos_x as f32 - (player_size as f32/2.),
        pos_y as f32 - (player_size as f32 /2.),
        player_size as f32,
        player_size as f32,
        DARKGREEN,
    );
}

fn draw_map(map_slice: &[bool]) {
    for pos_x in 0..MAP_SIZE_X {
        for pos_y in 0..MAP_SIZE_Y {
            if map_slice[(pos_x + pos_y * MAP_SIZE_X) as usize] {
                draw_rectangle(
                    pos_x as f32 * MAP_SIZE as f32,
                    pos_y as f32 * MAP_SIZE as f32,
                    MAP_SIZE as f32,
                    MAP_SIZE as f32,
                    DARKGRAY,
                );
            }
        }
    }
}

fn get_user_input(player_size : u32, pos_x: &mut u32, pos_y : &mut u32) {
    if is_key_down(KeyCode::Right) {
        if (*pos_x + (player_size as f32/2.) as u32) < 512 {
            *pos_x += 8;
        } else {
            *pos_x = 512 - (player_size as f32/2.) as u32;
        }
    } else if is_key_down(KeyCode::Left) {
        if (*pos_x - 8) >= (player_size as f32/2.) as u32 {
            *pos_x -= 8;
        } else {
            *pos_x = (player_size as f32/2.) as u32;
        }
    } else if is_key_down(KeyCode::Up) {
        if (*pos_y - 8) >= (player_size as f32/2.) as u32 {
            *pos_y -= 8;
        } else {
            *pos_y = (player_size as f32/2.) as u32;
        }
    } else if is_key_down(KeyCode::Down) {
        if (*pos_y + (player_size as f32/2.) as u32) < 512 {
            *pos_y += 8;
        } else {
            *pos_y = 512 - (player_size as f32/2.) as u32;
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut pos_x_player : u32 = 300;
    let mut pos_y_player : u32 = 300;

    let player_size : u32 = 16;

    loop {
        clear_background(LIGHTGRAY);
        get_user_input(player_size,&mut pos_x_player, &mut pos_y_player);
        draw_map(&MAP);
        move_player(player_size, pos_x_player, pos_y_player);

        next_frame().await
    }
}