extern crate piston_window;
extern crate rand;

use piston_window::{Button, PistonWindow, PressEvent, UpdateEvent, WindowSettings};

use draw::BLOCK_SIZE;
use game::{Game, GAME_HEIGHT, GAME_WIDTH};

mod draw;
mod game;
mod snake;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [GAME_WIDTH * BLOCK_SIZE, GAME_HEIGHT * BLOCK_SIZE])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(GAME_WIDTH, GAME_HEIGHT);
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
