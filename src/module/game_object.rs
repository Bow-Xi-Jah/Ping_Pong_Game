use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::module::const_values::*;

pub struct GameObject {
    vertical :i32,
    horizontal :i32,
    width :u32,
    height : u32,
    x_velocity : i32,
    y_velocity : i32,
    caracter_attibute : CharacterAttribute,
}
#[derive(PartialEq, Clone, Copy)]
pub enum CharacterAttribute {
    Player1,
    Player2,
    NPC,
}

impl GameObject {
    pub fn new(vertical: i32,
           horizontal: i32,
           width: u32,
           height: u32,
           x_velocity: i32,
           y_velocoty: i32,
           caracter_attribute: CharacterAttribute) -> GameObject {
        GameObject { vertical: vertical,
               horizontal: horizontal,
               width: width,
               height: height,
               x_velocity: x_velocity,
               y_velocity: y_velocoty,
               caracter_attibute: caracter_attribute}
    }

    pub fn control (&mut self, state: &KeyboardState<'_>) {
        if self.caracter_attibute == CharacterAttribute::Player1{
            if state.is_scancode_pressed(Scancode::W){
                if self.vertical > 0 {self.vertical -= self.y_velocity;}
            }

            if state.is_scancode_pressed(Scancode::S){
                if self.vertical < ((WINDOW_HEIGHT - self.height) as i32) {self.vertical += self.y_velocity;}
            }

            if state.is_scancode_pressed(Scancode::A){
                self.horizontal -= self.x_velocity;
            }

            if state.is_scancode_pressed(Scancode::D){
                self.horizontal += self.x_velocity;
            }
        }

        else if self.caracter_attibute == CharacterAttribute::Player2 {
            if state.is_scancode_pressed(Scancode::Up){
                if self.vertical > 0 {self.vertical -= 5;}
            }

            if state.is_scancode_pressed(Scancode::Down){
                if self.vertical < ((WINDOW_HEIGHT - self.height) as i32) {self.vertical += 5;}
            }

            if state.is_scancode_pressed(Scancode::Left){
                self.horizontal -= self.x_velocity;
            }

            if state.is_scancode_pressed(Scancode::Right){
                self.horizontal += self.x_velocity;
            }
        }

        else {
            panic!("unexpected attribute of character found");
        }

    }

    pub fn bounce_detection(&self, bar:&GameObject) -> bool{
        // if (self.vertical <= WINDOW_WIDTH as i32 / 2 ){
        //     (self.horizontal <= (bar.horizontal + (bar.width as i32))) & ((self.vertical + self.height as i32) >= bar.vertical) & (self.vertical <= (bar.vertical + bar.height as i32))
        // }
        // else{
        //     (self.horizontal >= (bar.horizontal - (bar.width as i32))) & ((self.vertical + self.height as i32) >= bar.vertical) & (self.vertical <= (bar.vertical + bar.height as i32))
        // }
        ((self.vertical + self.height as i32) >= bar.vertical) & (self.vertical <= (bar.vertical + bar.height as i32))
    }

    pub fn bounce (&mut self, bar1: &GameObject, bar2: &GameObject, state: &KeyboardState<'_>){
        if (self.horizontal <= (bar1.horizontal + (bar1.width as i32))) & self.bounce_detection(bar1){
            self.x_velocity = -1 * self.x_velocity;

            if state.is_scancode_pressed(Scancode::W) {
                self.y_velocity += 1;
            }

            if state.is_scancode_pressed(Scancode::S) {
                self.y_velocity -= 1;
            }
        }

        if (self.horizontal >= (bar2.horizontal - (bar2.width as i32))) & self.bounce_detection(bar2){
            self.x_velocity = -1 * self.x_velocity;

            if state.is_scancode_pressed(Scancode::Up) {
                self.y_velocity += 1;
            }

            if state.is_scancode_pressed(Scancode::Down) {
                self.y_velocity -= 1;
            }
        }

        if (self.vertical <= 0) | (self.vertical + (self.height as i32) >= (WINDOW_HEIGHT as i32)){
            self.y_velocity = -1 * self.y_velocity;
        }
    }

    pub fn auto_move (&mut self){
        self.horizontal += self.x_velocity;
        self.vertical += self.y_velocity;
    }

    pub fn goal_sequence(&mut self){
        if(self.horizontal <= 0)| ((self.horizontal + self.width as i32)  >= WINDOW_WIDTH as i32){
            self.y_velocity = 4;
            self.horizontal = WINDOW_WIDTH as i32 / 2;
            self.vertical = WINDOW_HEIGHT as i32 / 2;
            std::thread::sleep(Duration::from_secs(1));
        }
    }

    pub fn speed_boost_p1 (&mut self, state: &KeyboardState<'_>, speed: u32){
        //let x_speed_original = self.x_velocity;
        //let y_speed_original = self.y_velocity;

        if state.is_scancode_pressed(Scancode::LShift){
            //self.x_velocity = speed;
            self.y_velocity = speed as i32;
        }

        if !state.is_scancode_pressed(Scancode::LShift){
            //self.x_velocity = x_speed_original;
            self.y_velocity = 5;
        }
    }

    pub fn speed_boost_p2 (&mut self, state: &KeyboardState<'_>, speed: u32){
        //let x_speed_original = self.x_velocity;
        //let y_speed_original = self.y_velocity;

        if state.is_scancode_pressed(Scancode::RShift){
            //self.x_velocity = speed;
            self.y_velocity = speed as i32;
        }

        if !state.is_scancode_pressed(Scancode::RShift){
            //self.x_velocity = x_speed_original;
            self.y_velocity = 5;
        }
    }

    pub fn power_attack(&self, state: &KeyboardState<'_>){
        //TODO
    }

    pub fn fill_rect_object(&self, canvas: &mut Canvas<Window>) -> Result<(), String>{
        canvas.fill_rect(Rect::new(self.horizontal, self.vertical, self.width, self.height))?;
        Ok(())
    }

    // pub fn convert_taple (&self) -> (i32, i32, u32, u32){
    //     (self.vertical, self.horizontal, self.width, self.height)
    // }
}