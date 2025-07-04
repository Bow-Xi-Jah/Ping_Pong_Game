extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

// use sdl2::rect::Rect;
// use sdl2::render::Canvas;
// use sdl2::video::Window;
// use sdl2::keyboard::KeyboardState;
// use sdl2::keyboard::Scancode;

use crate::module::game_object::GameObject;

mod module;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

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

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        
    }

    Ok(())
}
