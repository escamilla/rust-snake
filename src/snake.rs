use std::collections::LinkedList;
use std::f64::consts::PI;

use piston_window::math::Matrix2d;
use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d, Transformed};

use draw::{draw_block, Block, BLOCK_SIZE};

const SNAKE_COLOR: Color = [0.34, 0.80, 0.17, 1.0];
const EYE_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const TONGUE_COLOR: Color = [1.00, 0.50, 0.67, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new() -> Self {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: 3, y: 3 });
        body.push_back(Block { x: 2, y: 3 });
        body.push_back(Block { x: 1, y: 3 });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, context: &Context, g2d: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, context, g2d);
        }
        self.draw_eyes(context, g2d);
        self.draw_tongue(context, g2d);
    }

    fn transform_based_on_direction(&self, context: &Context) -> Matrix2d {
        let (head_x, head_y): (u32, u32) = self.position();
        let center_x = ((head_x * BLOCK_SIZE) as f64) + ((BLOCK_SIZE as f64) / 2.0);
        let center_y = ((head_y * BLOCK_SIZE) as f64) + ((BLOCK_SIZE as f64) / 2.0);
        context
            .transform
            .trans(center_x, center_y)
            .rot_rad(match self.direction {
                Direction::Up => 0.0,
                Direction::Right => PI * 0.5,
                Direction::Down => PI,
                Direction::Left => PI * 1.5,
            })
    }

    fn draw_eyes(&self, context: &Context, g2d: &mut G2d) {
        let pixel_size = (BLOCK_SIZE as f64) * 0.1;
        let transform = self.transform_based_on_direction(context);

        rectangle(
            EYE_COLOR,
            [
                pixel_size * -2.75,
                pixel_size * -2.5,
                pixel_size * 2.0,
                pixel_size * 2.0,
            ],
            transform,
            g2d,
        );

        rectangle(
            EYE_COLOR,
            [
                pixel_size * 0.75,
                pixel_size * -2.5,
                pixel_size * 2.0,
                pixel_size * 2.0,
            ],
            transform,
            g2d,
        );
    }

    fn draw_tongue(&self, context: &Context, g2d: &mut G2d) {
        let pixel_size = (BLOCK_SIZE as f64) * 0.1;
        let transform = self.transform_based_on_direction(context);

        rectangle(
            TONGUE_COLOR,
            [
                pixel_size * -0.5,
                ((BLOCK_SIZE as f64) * -0.5) - (pixel_size * 2.0),
                pixel_size,
                (pixel_size * 2.0),
            ],
            transform,
            g2d,
        );

        rectangle(
            TONGUE_COLOR,
            [
                (pixel_size * -0.5) - pixel_size,
                ((BLOCK_SIZE as f64) * -0.5) - (pixel_size * 3.0),
                pixel_size,
                pixel_size,
            ],
            transform,
            g2d,
        );

        rectangle(
            TONGUE_COLOR,
            [
                (pixel_size * -0.5) + pixel_size,
                ((BLOCK_SIZE as f64) * -0.5) - (pixel_size * 3.0),
                pixel_size,
                pixel_size,
            ],
            transform,
            g2d,
        );
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn position(&self) -> (u32, u32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, direction: Option<Direction>) {
        if direction.is_some() {
            self.direction = direction.unwrap();
        }

        let (next_x, next_y) = self.next_head(Some(self.direction));
        let new_block = Block {
            x: next_x,
            y: next_y,
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn next_head(&self, direction: Option<Direction>) -> (u32, u32) {
        let (current_x, current_y) = self.position();

        let moving_direction = match direction {
            Some(d) => d,
            None => self.direction,
        };

        match moving_direction {
            Direction::Up => (current_x, current_y - 1),
            Direction::Down => (current_x, current_y + 1),
            Direction::Left => (current_x - 1, current_y),
            Direction::Right => (current_x + 1, current_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let tail_block = self.tail.clone().unwrap();
        self.body.push_back(tail_block);
    }

    pub fn overlaps_tail(&self, x: u32, y: u32) -> bool {
        let mut i = 0;
        for block in &self.body {
            if block.x == x && block.y == y {
                return true;
            }

            i += 1;
            if i == self.body.len() - 1 {
                break;
            }
        }
        false
    }
}
