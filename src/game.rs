use piston_window::types::Color;
use piston_window::{clear, rectangle, Context, G2d, Key};
use rand::{thread_rng, Rng};

use draw::{draw_rectangle, BLOCK_SIZE};
use snake::{Direction, Snake};

const BACKGROUND_COLOR: Color = [0.1, 0.1, 0.1, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const APPLE_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
const STEM_COLOR: Color = [0.60, 0.39, 0.22, 1.0];
const LEAF_COLOR: Color = [0.34, 0.80, 0.17, 1.0];
const GAME_OVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.5;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    width: u32,
    height: u32,
    snake: Snake,
    apple_exists: bool,
    apple_x: u32,
    apple_y: u32,
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let mut game = Game {
            width,
            height,
            snake: Snake::new(),
            apple_exists: false,
            apple_x: 0,
            apple_y: 0,
            game_over: false,
            waiting_time: 0.0,
        };
        game.add_apple();
        game
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let new_direction: Option<Direction> = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if new_direction.is_some() && new_direction.unwrap() == self.snake.direction().opposite() {
            return;
        }

        self.update_snake(new_direction);
    }

    pub fn update_snake(&mut self, direction: Option<Direction>) {
        if self.check_if_snake_is_alive(direction) {
            self.snake.move_forward(direction);
            self.check_if_snake_has_eaten();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    pub fn draw(&self, context: &Context, g2d: &mut G2d) {
        clear(BACKGROUND_COLOR, g2d);

        self.draw_border(context, g2d);

        if self.apple_exists {
            self.draw_apple(context, g2d);
        }

        self.snake.draw(context, g2d);

        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.width, self.height, context, g2d);
        }
    }

    fn draw_border(&self, context: &Context, g2d: &mut G2d) {
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, g2d);
        draw_rectangle(
            BORDER_COLOR,
            0,
            self.height - 1,
            self.width,
            1,
            context,
            g2d,
        );
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, g2d);
        draw_rectangle(
            BORDER_COLOR,
            self.width - 1,
            0,
            1,
            self.height,
            context,
            g2d,
        );
    }

    fn draw_apple(&self, context: &Context, g2d: &mut G2d) {
        let apple_offset: f64 = (BLOCK_SIZE as f64) * 0.2;
        let apple_size: f64 = (BLOCK_SIZE as f64) * 0.6;

        rectangle(
            APPLE_COLOR,
            [
                ((self.apple_x * BLOCK_SIZE) as f64) + apple_offset,
                ((self.apple_y * BLOCK_SIZE) as f64) + apple_offset,
                apple_size,
                apple_size,
            ],
            context.transform,
            g2d,
        );

        let stem_width = (BLOCK_SIZE as f64) * 0.1;
        let stem_height = (BLOCK_SIZE as f64) * 0.2;
        let stem_offset_x = ((BLOCK_SIZE as f64) * 0.5) - (stem_width / 2.0);

        rectangle(
            STEM_COLOR,
            [
                ((self.apple_x * BLOCK_SIZE) as f64) + stem_offset_x,
                ((self.apple_y * BLOCK_SIZE) as f64) + ((BLOCK_SIZE as f64) * 0.1),
                stem_width,
                stem_height,
            ],
            context.transform,
            g2d,
        );

        rectangle(
            STEM_COLOR,
            [
                ((self.apple_x * BLOCK_SIZE) as f64) + stem_offset_x - stem_width,
                ((self.apple_y * BLOCK_SIZE) as f64),
                stem_width,
                (BLOCK_SIZE as f64) * 0.1,
            ],
            context.transform,
            g2d,
        );

        let leaf_size = (BLOCK_SIZE as f64) * 0.1;

        rectangle(
            LEAF_COLOR,
            [
                ((self.apple_x * BLOCK_SIZE) as f64) + stem_offset_x + stem_width,
                ((self.apple_y * BLOCK_SIZE) as f64) + ((BLOCK_SIZE as f64) * 0.1),
                leaf_size,
                leaf_size,
            ],
            context.transform,
            g2d,
        );

        rectangle(
            LEAF_COLOR,
            [
                ((self.apple_x * BLOCK_SIZE) as f64) + stem_offset_x + stem_width + stem_width,
                ((self.apple_y * BLOCK_SIZE) as f64),
                leaf_size,
                leaf_size,
            ],
            context.transform,
            g2d,
        );
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.apple_exists {
            self.add_apple();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new();
        self.add_apple();
        self.game_over = false;
        self.waiting_time = 0.0;
    }

    pub fn add_apple(&mut self) {
        let mut rng = thread_rng();

        let (apple_x, apple_y) = loop {
            let x: u32 = rng.gen_range(0, self.width);
            let y: u32 = rng.gen_range(0, self.height);
            if !self.overlaps_border(x, y) && !self.snake.overlaps_tail(x, y) {
                break (x, y);
            }
        };

        self.apple_x = apple_x;
        self.apple_y = apple_y;
        self.apple_exists = true;
    }

    pub fn overlaps_border(&self, x: u32, y: u32) -> bool {
        x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1
    }

    pub fn check_if_snake_has_eaten(&mut self) {
        if self.apple_exists {
            let (head_x, head_y): (u32, u32) = self.snake.position();
            if head_x == self.apple_x && head_y == self.apple_y {
                self.apple_exists = false;
                self.snake.restore_tail();
            }
        }
    }

    pub fn check_if_snake_is_alive(&self, direction: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(direction);

        if self.snake.overlaps_tail(next_x, next_y) {
            return false;
        }

        !self.overlaps_border(next_x, next_y)
    }
}
