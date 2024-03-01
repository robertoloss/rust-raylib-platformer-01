extern crate raylib;
use raylib::prelude::*;
use crate::map::*;

#[derive(Debug)]
pub struct MaxVelY {
    //jump: f32,
    pub fall: f32,
}

#[derive(Debug)]
pub struct Player {
    pub position: Vector2,
    pub velocity: Vector2,
    pub size: Vector2,
    pub max_velocity: MaxVelY,
    pub gravity: f32,
    pub on_ground: bool,
    pub going_up: bool,
    pub jump_start: f32,
    pub jump_max: f32,
    pub x_increment: f32,
    pub going_right: bool,
    pub going_left: bool,
    pub on_platform: bool,
    pub platform_go_right: bool,
    pub cling: bool,
    pub on_wall: bool,
    pub can_cling: bool,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    NoDirection
}

impl Player {
    pub fn check_collision(&mut self, tile: &Tile) -> (bool,Direction) {
        let x_overlap = self.position.x + self.size.x > tile.position.x &&
                        self.position.x < tile.position.x + tile.size.x;

        let y_overlap = (self.position.y + self.size.y) > tile.position.y &&
                        self.position.y < (tile.position.y + tile.size.y);
        
        //Bottom Collision
        if  self.position.y + self.size.y + self.velocity.y > tile.position.y &&
            self.position.y + self.size.y + self.velocity.y < tile.position.y + 12.  &&
            self.position.y < tile.position.y &&
            x_overlap &&
            self.velocity.y > 0.

        {
            self.velocity.y = 0.0;
            self.position.y = tile.position.y - (self.size.y);
            //println!("Bottom Collision");
            return (true, Direction::Bottom)
        }

                
        //Right Collision
        if  self.position.x + self.size.x + self.velocity.x > tile.position.x &&
            self.position.x + self.velocity.x < tile.position.x &&
            y_overlap &&
            (self.velocity.y >= 0. || self.position.y < (tile.position.y + tile.size.y) - 10.0) 
        {
            self.velocity.x = 0.0;
            self.position.x = tile.position.x - (self.size.x );
           // println!("Right Collision");
            return ( true, Direction::Right )
        } 
        
        //Left Collision
        if  self.position.x < tile.position.x + tile.size.x &&
            (self.position.x + self.size.x) + self.velocity.x > tile.position.x + tile.size.x &&
            y_overlap &&
            self.jump_start != self.position.y
        {
            self.velocity.x = 0.0;
            self.position.x = tile.position.x + tile.size.x ;
            //println!("Left Collision");
            return ( true, Direction::Left )
        }

        //Top Collision
        if  self.position.y + self.velocity.y < tile.position.y + tile.size.y &&
            self.position.y + self.size.y > tile.position.y + tile.size.y &&
            x_overlap &&
            self.position.x < (tile.position.x + tile.size.x) - self.x_increment &&
            self.position.x + self.size.x > tile.position.x + self.x_increment &&
            self.velocity.y < 0. 
        {
            self.velocity.y = 0.0;
            self.position.y = tile.position.y + tile.size.y;
            //println!("Top Collision");
            return ( true, Direction::Top )
        }
        (false,Direction::NoDirection)
    }
}
