extern crate piston_window;
extern crate rand;

use piston_window::{Button, PistonWindow, PressEvent, UpdateEvent, WindowSettings};

use draw::convert_coord_to_pixels;
use game::Game;

mod draw;
mod game;
mod snake;

fn main() {
    let (width, height): (u32, u32) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [
            convert_coord_to_pixels(width),
            convert_coord_to_pixels(height),
        ],
    ).exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |context, g2d| {
            game.draw(&context, g2d);
        });

        event.update(|args| {
            game.update(args.dt);
        });
    }
}
