extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

struct GameObject {
    vertical :i32,
    horizontal :i32,
    width :u32,
    height : u32,
    x_velocity : i32,
    y_velocity : i32,
}

impl GameObject {
    fn new(vertical: i32,
           horizontal: i32,
           width: u32,
           height: u32,
           x_velocity: i32,
           y_velocoty: i32) -> Self {
        Self { vertical: vertical,
               horizontal: horizontal,
               width: width,
               height: height,
               x_velocity: x_velocity,
               y_velocity: y_velocoty}
    }

    fn control_p2 (&mut self, state: &KeyboardState<'_>) {
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

    fn control_p1 (&mut self, state: &KeyboardState<'_>) {
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

    fn bounce_detection(&self, bar:&GameObject) -> bool{
        // if (self.vertical <= WINDOW_WIDTH as i32 / 2 ){
        //     (self.horizontal <= (bar.horizontal + (bar.width as i32))) & ((self.vertical + self.height as i32) >= bar.vertical) & (self.vertical <= (bar.vertical + bar.height as i32))
        // }
        // else{
        //     (self.horizontal >= (bar.horizontal - (bar.width as i32))) & ((self.vertical + self.height as i32) >= bar.vertical) & (self.vertical <= (bar.vertical + bar.height as i32))
        // }
        ((self.vertical + self.height as i32) >= bar.vertical) & (self.vertical <= (bar.vertical + bar.height as i32))
    }

    fn bounce (&mut self, bar1: &GameObject, bar2: &GameObject){
        if ((self.horizontal <= (bar1.horizontal + (bar1.width as i32))) & self.bounce_detection(bar1))
         | ((self.horizontal >= (bar2.horizontal - (bar2.width as i32))) & self.bounce_detection(bar2)){
            self.x_velocity = -1 * self.x_velocity;
        }

        if (self.vertical <= 0) | (self.vertical + (self.height as i32) >= (WINDOW_HEIGHT as i32)){
            self.y_velocity = -1 * self.y_velocity;
        }
    }

    fn auto_move (&mut self){
        self.horizontal += self.x_velocity;
        self.vertical += self.y_velocity;
    }

    fn goal_sequence(&mut self){
        if(self.horizontal <= 0)| (self.horizontal >= WINDOW_WIDTH as i32){
            self.horizontal = WINDOW_WIDTH as i32 / 2;
            self.vertical = WINDOW_HEIGHT as i32 / 2;
        }
    }

    fn speed_boost_p1 (&mut self, state: &KeyboardState<'_>, speed: u32){
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

    fn speed_boost_p2 (&mut self, state: &KeyboardState<'_>, speed: u32){
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

    fn fill_rect_object(&self, canvas: &mut Canvas<Window>) -> Result<(), String>{
        canvas.fill_rect(Rect::new(self.horizontal, self.vertical, self.width, self.height))?;
        Ok(())
    }

    // fn convert_taple (&self) -> (i32, i32, u32, u32){
    //     (self.vertical, self.horizontal, self.width, self.height)
    // }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust-SDL2", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;


    let mut event_pump = sdl_context.event_pump()?;


    let mut bar_1 = GameObject::new(250, 30, 20, 100, 0, 5);

    let mut bar_2 = GameObject::new(250, (WINDOW_WIDTH as i32) - 50, 20, 100, 0, 5);

    let mut ball = GameObject::new(250, 400, 20, 20, 2, 2);

    //let (mut x, mut y) = (30, 250);
    // let (mut x, mut y) = (0, 0);

    'running: loop {
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let state = event_pump.keyboard_state();

        bar_1.control_p1(&state);
        bar_2.control_p2(&state);

        bar_1.speed_boost_p1(&state, 10);
        bar_2.speed_boost_p2(&state, 10);

        ball.bounce(&bar_1, &bar_2);
        ball.goal_sequence();

        ball.auto_move();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));


        bar_1.fill_rect_object(&mut canvas)?;
        bar_2.fill_rect_object(&mut canvas)?;
        ball.fill_rect_object(&mut canvas)?;



        //canvas.fill_rect(Rect::new(bar_1.horizontal, bar_1.vertical, bar_1.width, bar_1.height))?;
        //canvas.fill_rect(Rect::new(bar_2.horizontal, bar_2.vertical, bar_2.width, bar_1.height))?;
        //canvas.fill_rect(Rect::new(ball.horizontal, ball.vertical, ball.width, ball.height))?;

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        
    }

    Ok(())
}
