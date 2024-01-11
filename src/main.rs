extern crate opengl_graphics;
extern crate piston;

use piston::{Button, ButtonArgs, ButtonState, Input, Loop, Motion, MouseButton, Events};
use piston::window::WindowSettings;
use piston_window::{Event, PistonWindow};
use std::collections::VecDeque;

use app::App;
use constants::RENDERER;

mod constants;
mod player;
mod game_state;
mod game;
mod app;
mod animation;

type Pos = (f64, f64);
type Size = (f64, f64);

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Four Wins", [800, 800])
        .graphics_api(RENDERER)
        .samples(2)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    
    let event_settings = piston_window::EventSettings {
        max_fps: 60,
        ups: 60,
        ups_reset: 2,
        swap_buffers: true,
        bench_mode: false,
        lazy: false,
    };
    
    let mut events = Events::new(event_settings);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let glyphs = window.load_font(assets.join("RobotoMono-Regular.ttf")).unwrap();

    let mut app = App::initial(glyphs);
    
    let mut frames: VecDeque<f64> = VecDeque::with_capacity(10);

    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Loop(loop_event) => {
                match loop_event {
                    Loop::Render(args) => {
                        if frames.len() == frames.capacity() {
                            frames.pop_back();
                        }
                        frames.push_front(args.ext_dt);

                        if frames.len() > 0 {
                            let fps: f64 = 1.0 / (frames.iter().sum::<f64>() / frames.len() as f64);
                            println!("{} FPS", fps as i64); 
                        }

                        window.draw_2d(&e, |c, g, d| {
                            app.render(&args, c, g, d);
                        });
                    }

                    // 120 fps
                    Loop::Update(args) => {
                        app.update(&args);
                    }

                    _ => (),
                }
            }

            Event::Input(
                Input::Button(
                    ButtonArgs {
                        state: ButtonState::Press,
                        button: Button::Mouse(MouseButton::Left),
                        ..
                    }), _) => {
                app.handle_click();
            }

            Event::Input(Input::Move(Motion::MouseCursor([x, y])), _) => {
                app.set_mouse_pos((x, y));
            }

            _ => ()
        };
    }
}
