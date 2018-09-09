use piston_window::types::Color;
use piston_window::{clear, Context, G2d, Key};
use rand::{thread_rng, Rng};

use draw::{draw_block, draw_rectangle};
use snake::{Direction, Snake};

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const GAME_OVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.5;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    width: u32,
    height: u32,

    snake: Snake,

    food_exists: bool,
    food_x: u32,
    food_y: u32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let mut game = Game {
            width,
            height,
            snake: Snake::new(),
            food_exists: false,
            food_x: 0,
            food_y: 0,
            game_over: false,
            waiting_time: 0.0,
        };
        game.add_food();
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

        if new_direction.unwrap() == self.snake.direction().opposite() {
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

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, g2d);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, context, g2d);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, g2d);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, context, g2d);

        self.snake.draw(context, g2d);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, context, g2d)
        }

        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.width, self.height, context, g2d);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new();
        self.add_food();
        self.game_over = false;
        self.waiting_time = 0.0;
    }

    pub fn add_food(&mut self) {
        let mut rng = thread_rng();

        let (new_x, new_y) = loop {
            let x: u32 = rng.gen_range(0, self.width);
            let y: u32 = rng.gen_range(0, self.height);
            if !self.overlaps_border(x, y) && !self.snake.overlaps_tail(x, y) {
                break (x, y);
            }
        };

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    pub fn overlaps_border(&self, x: u32, y: u32) -> bool {
        x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1
    }

    pub fn check_if_snake_has_eaten(&mut self) {
        if self.food_exists {
            let (head_x, head_y): (u32, u32) = self.snake.position();
            if head_x == self.food_x && head_y == self.food_y {
                self.food_exists = false;
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
