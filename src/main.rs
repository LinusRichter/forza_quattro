extern crate opengl_graphics;
extern crate piston;

use piston::{Button, ButtonArgs, ButtonState, Input, Loop, Motion, MouseButton};
use piston::window::WindowSettings;
use piston_window::{Event, PistonWindow};

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
        .vsync(true)
        .graphics_api(RENDERER)
        .samples(8)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let glyphs = window.load_font(assets.join("RobotoMono-Regular.ttf")).unwrap();

    let mut app = App::initial(glyphs);

    while let Some(e) = window.next() {
        match e {
            Event::Loop(Loop::Render(args)) => {
                window.draw_2d(&e, |c, g, d| {
                    app.render(&args, c, g, d);
                });
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
