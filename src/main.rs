extern crate raylib;
use rand::Rng;
use raylib::prelude::*;
mod player;
mod map;
use map::*;
use player::*;
 
fn main() {
    let screen_width = 960;
    let screen_height = 640;

    let gravity = 15. / 1000.;

    let mut rng = rand::thread_rng();

    let map = &get_map();

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Rust rules!")
        .build();

    let mut player = Player {
        position :  Vector2 { x: 80., y: 40. },
        velocity : Vector2 {x: 0., y: 0.},
        max_velocity: MaxVelY {  fall: 20. },
        gravity,
        size : Vector2 { x: 40., y: 40. },
        on_ground: true,
        cling: false,
        going_up: false,
        jump_start: 0.,
        jump_max: 20.,
        x_increment: 4.,
        going_left: false,
        going_right: false,
        on_platform: false,
        platform_go_right: false,
        on_wall: false,
        can_cling: true,
    };
    let platform_velocity = 1.0;

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
    
    let mut platforms : Vec<&mut Tile> = vec![];
    
    let mut platform: Tile = Tile::new(5, 5);
    platform.size.x = platform.size.x * 5.0;
    //let mut platform2: Tile = Tile::new(6, 5);
    //let mut platform3: Tile = Tile::new(7, 5);
    //let mut platform4: Tile = Tile::new(8, 5);
    let mut platform_go_right = true;

    platforms.push(&mut platform);
    //platforms.push(&mut platform2);
    //platforms.push(&mut platform3);
    //platforms.push(&mut platform4);



    while !rl.window_should_close() {

        //println!("player {:?}", player);

        if  rl.is_key_down(KeyboardKey::KEY_LEFT)  &&
            !rl.is_key_down(KeyboardKey::KEY_RIGHT) 
        {
            player.going_left = true;
        }
        if  rl.is_key_down(KeyboardKey::KEY_RIGHT) &&
            !rl.is_key_down(KeyboardKey::KEY_LEFT) 
        {
            player.going_right = true;
        }
        if  (!rl.is_key_down(KeyboardKey::KEY_LEFT) && 
            !rl.is_key_down(KeyboardKey::KEY_RIGHT)) ||
            (rl.is_key_down(KeyboardKey::KEY_LEFT) && 
            rl.is_key_down(KeyboardKey::KEY_RIGHT))
        {
            player.velocity.x = 0.;
            player.going_left = false;
            player.going_right = false;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_Z) {
            player.velocity.y = -9.;
            if player.on_wall { player.can_cling = false; }
            player.cling = false;
            player.jump_start = player.position.y;
        }
        if rl.is_key_released(KeyboardKey::KEY_Z) && player.velocity.y < 0. {
            player.velocity.y -= player.velocity.y / 2.;
            player.can_cling = true;
        }
        if  rl.is_key_down(KeyboardKey::KEY_X) && player.can_cling {
            player.cling = true;
        } else {
            player.cling = false;
        }
        if !player.cling {
            player.on_wall = false;
            player.gravity = gravity;
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

        if player.going_left {
            if !player.on_wall {
                player.velocity.x = -player.x_increment;
            }
        }
        if player.going_right {
            if !player.on_wall {
                player.velocity.x = player.x_increment;
            }
        }
        if player.velocity.x > 0.0 && rl.is_key_down(KeyboardKey::KEY_LEFT) {
            player.velocity.x = -player.x_increment; 
        }
        

        player.position.y += player.velocity.y * rl.get_frame_time() * fps;
        player.position.x += player.velocity.x * rl.get_frame_time() * fps;

        for platform in &mut platforms {
            if platform_go_right {
                platform.position.x += platform_velocity * rl.get_frame_time() * fps; 
            } else {
                platform.position.x -= platform_velocity * rl.get_frame_time() * fps;
            }
            if platform.position.x < 4.0 * platform.size.y { platform_go_right = true }
            if platform.position.x + platform.size.x > 20.0 * platform.size.y { platform_go_right = false }
            
            let platform_collision = player.check_collision(platform);
            if platform_collision.0 == true && platform_collision.1 == Direction::Bottom {
                //println!("player on platform {}", rng.gen_range(1..=100));
                player.on_platform = true;
                player.platform_go_right = platform_go_right;
            } else {
                //if player.velocity.y < 0.0 || player.velocity.y > player.gravity {
                //    player.on_platform = false;
                //}
            }
        }

        
        fn stick_to_the_wall(player: &mut Player) {
            if player.cling {
                player.gravity = 0.;
                player.velocity.y = 0.;
                player.on_wall = true;
            }
        }

        collisions.iter().for_each(|c| { 
            let collision = player.check_collision(c); 
            if collision.0 == true {
                match collision.1 {
                    Direction::Bottom => {
                        player.on_platform = false;
                    }
                    Direction::Top => {}
                    Direction::Left => {
                        stick_to_the_wall(&mut player);
                        //println!("{} {}", player.gravity, rng.gen_range(1..=100))
                    }                     
                    Direction::Right => {
                        stick_to_the_wall(&mut player);
                    }
                    Direction::NoDirection => {}
                }
            }
        });

        let d = &mut rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        collisions.iter().for_each(|c| d.draw_rectangle_v(&c.position, &c.size, Color::BLUE));
        for i in 0..platforms.len() {
            let platform = &mut platforms[i];
            d.draw_rectangle_v(platform.position, platform.size, Color::PURPLE);
        }
        d.draw_rectangle_v(&player.position, &player.size, Color::MAROON);
        d.draw_text(format!("pl.platf_go_right: {}",  player.platform_go_right).as_str(), 10, 10, 14, Color::YELLOW);
    }
}
