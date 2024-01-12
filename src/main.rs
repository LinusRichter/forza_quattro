extern crate opengl_graphics;
extern crate piston;

use gfx_graphics::{Texture, Flip, TextureSettings};
use piston::{Button, ButtonArgs, ButtonState, Input, Loop, Motion, MouseButton, Events};
use piston::window::WindowSettings;
use piston_window::{Event, PistonWindow, G2dTexture};
use std::collections::VecDeque;

use app::App;
use constants::{RENDERER, EVENT_SETTINGS};

mod constants;
mod player;
mod game_state;
mod game;
mod app;
mod animation;
mod gravity_floor_state;

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
    
    let mut events = Events::new(EVENT_SETTINGS);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap_or_else(|e| panic!("Failed to find asset folder: {}", e));

    let glyphs = window.load_font(assets.join("RobotoMono-Regular.ttf")).unwrap();
    
    let tile = assets.join("tile.png");

    let tile: G2dTexture = Texture::from_path(
            &mut window.create_texture_context(),
            &tile,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

    let mut app = App::initial(glyphs, tile);
    
    let mut frames: VecDeque<f64> = VecDeque::with_capacity(10);

    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Loop(loop_event) => {
                match loop_event {

                    // Unknown FPS (wonky)
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
