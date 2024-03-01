extern crate raylib;
use raylib::prelude::*;
mod player;
mod map;
use map::*;
use player::*;
 
fn main() {
    let screen_width = 960;
    let screen_height = 640;

    let map = &get_map();

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Rust rules!")
        .build();

    let mut player = Player {
        position :  Vector2 { x: 80., y: 40. },
        velocity : Vector2 {x: 0., y: 0.},
        max_velocity: MaxVelY {  fall: 20. },
        gravity: 15. / 1000.,
        size : Vector2 { x: 40., y: 40. },
        on_ground: true,
        going_up: false,
        jump_start: 0.,
        jump_max: 20.,
        x_increment: 4.,
        going_left: false,
        going_right: false,
        on_platform: false,
        platform_go_right: false,
    };
    let platform_velocity = 2.0;

    player.position = Vector2 {
        x: 200.,
        y: screen_height as f32 - (player.size.y * 2.0) 
    };

    let fps = 60.;


    let mut collisions : Vec<Tile> = vec![];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 1 {
                let tile = Tile::new(j,i);
                collisions.push(tile);
            }
        }
    }
    
    
    let mut platform: Tile = Tile::new(5, 5);
    let mut platform_go_right = true;



    while !rl.window_should_close() {

        //println!("player {:?}", player);

        if  rl.is_key_down(KeyboardKey::KEY_LEFT)  &&
            !rl.is_key_down(KeyboardKey::KEY_RIGHT) 
        {
            player.velocity.x = -player.x_increment;
            player.going_left = true;
        }
        if  rl.is_key_down(KeyboardKey::KEY_RIGHT) &&
            !rl.is_key_down(KeyboardKey::KEY_LEFT) 
        {
            player.velocity.x = player.x_increment;
            player.going_right = true;
        }
        if  !rl.is_key_down(KeyboardKey::KEY_LEFT) && 
            !rl.is_key_down(KeyboardKey::KEY_RIGHT)
        {
            player.velocity.x = 0.;
            player.going_left = false;
            player.going_right = false;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_Z) {
            player.velocity.y = -9.;
            player.jump_start = player.position.y;
        }
        if rl.is_key_released(KeyboardKey::KEY_Z) && player.velocity.y < 0. {
            player.velocity.y -= player.velocity.y / 2.;
        }
        if player.velocity.y < player.max_velocity.fall {
            player.velocity.y += player.gravity;
        }

        if player.on_platform {
            if player.platform_go_right {
                player.velocity.x += platform_velocity;
            } else {
                player.velocity.x -= platform_velocity;
            }
        }
        

        player.position.y += player.velocity.y * rl.get_frame_time() * fps;
        player.position.x += player.velocity.x * rl.get_frame_time() * fps;

        if platform_go_right {
            platform.position.x += platform_velocity * rl.get_frame_time() * fps; 
        } else {
            platform.position.x -= platform_velocity * rl.get_frame_time() * fps;
        }
        if platform.position.x < 4.0 * platform.size.x { platform_go_right = true }
        if platform.position.x > 19.0 * platform.size.x { platform_go_right = false }
        
        let platform_collision = player.check_collision(&platform);
        if platform_collision.0 == true && platform_collision.1 == Direction::Bottom {
            player.on_platform = true;
            player.platform_go_right = platform_go_right;
        } else {
            player.on_platform = false;
        }

        collisions.iter().for_each(|c| { player.check_collision(c); });

        let d = &mut rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        collisions.iter().for_each(|c| d.draw_rectangle_v(&c.position, &c.size, Color::BLUE));
        d.draw_rectangle_v(&platform.position, &platform.size, Color::PURPLE);
        d.draw_rectangle_v(&player.position, &player.size, Color::MAROON);
    }
}
