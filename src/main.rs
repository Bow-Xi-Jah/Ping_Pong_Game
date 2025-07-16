extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
//use std::path::Path;
// use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::ttf;
use debounce::EventDebouncer;

// use sdl2::rect::Rect;
// use sdl2::render::Canvas;
// use sdl2::video::Window;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;

use crate::module::game_object::{GameObject, CharacterAttribute};
use crate::module::const_values::*;
use crate::module::game_state::GameState;

mod module;

const TITLE: &str = "src/assets/image";
const FONT: &str = "src/assets/fonts/Square.ttf";

// #[derive(PartialEq, Clone, Copy)]
// enum GameState {
//     GAME_RUNNING,
//     GAME_PAUSED,
//     GAME_QUIT
// }





pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;


    //let delay = Duration::from_millis(10);
    // let debouncer = EventDebouncer::new(delay, move |data: String| {
    //     println!("{}", data);
    // });
    let debouncer = EventDebouncer::new(DELAY, |mut game_state: GameState|{
        game_state.pause();
    });

    let debouncer2 = EventDebouncer::new(DELAY, |data:String| {
        print!("{} ", data);
    });


    


    // let font = ttf_context.load_font(FONT, 20)?;
    // let font_surface = font
    //     .render("ABC")
    //     .blended(WHITE)
    //     .map_err(|e| e.to_string())?;

    let mut current_state = GameState::GAME_RUNNING;
    

    let window = video_subsystem
        .window("Rust-SDL2", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let text_creator = canvas.texture_creator();
    //let img = Path::new(TITLE);
    let font = ttf_context.load_font(FONT, 20)?;

    let surface = font
        .render("PAUSE")
        .blended(Color::WHITE)
        .map_err(|e| e.to_string())?;

    let texture = text_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let target = Rect::new(WINDOW_WIDTH as i32 / 2 - surface.width() as i32 / 2,
                                 WINDOW_HEIGHT as i32 / 2 - surface.height() as i32 / 2,
                                 surface.width(),
                                 surface.height());


    let mut event_pump = sdl_context.event_pump()?;

    let mut bar_1 = GameObject::new(250, 30                        , 20, 100, 0, 5, CharacterAttribute::Player1);

    let mut bar_2 = GameObject::new(250, (WINDOW_WIDTH as i32) - 50, 20, 100, 0, 5, CharacterAttribute::Player2);

    let mut ball = GameObject::new(250, 400                        , 20, 20, 4, 4, CharacterAttribute::NPC);

    let mut prev_p_pressed = false;
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
        let p_pressed = state.is_scancode_pressed(Scancode::P);

        // Pキーが押された瞬間だけ実行
        if p_pressed && !prev_p_pressed {
            current_state.pause();
            //debouncer.put(current_state);
            //print!("P_key_pressed");
        }
        prev_p_pressed = p_pressed;

        if current_state == GameState::GAME_RUNNING {
            bar_1.control(&state);
            bar_2.control(&state);

            bar_1.speed_boost_p1(&state, 10);
            bar_2.speed_boost_p2(&state, 10);

            ball.bounce(&bar_1, &bar_2, &state);
            ball.goal_sequence();
            ball.auto_move();

            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            canvas.set_draw_color(Color::WHITE);


            bar_1.fill_rect_object(&mut canvas)?;
            bar_2.fill_rect_object(&mut canvas)?;
            ball.fill_rect_object(&mut canvas)?;


        }

        if current_state == GameState::GAME_PAUSED {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            canvas.set_draw_color(Color::WHITE);


            bar_1.fill_rect_object(&mut canvas)?;
            bar_2.fill_rect_object(&mut canvas)?;
            ball.fill_rect_object(&mut canvas)?;

            canvas.copy(&texture, None, Some(target))?;

        }


        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        
    }

    Ok(())
}




