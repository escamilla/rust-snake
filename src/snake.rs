use std::collections::LinkedList;
use std::f64::consts::PI;

use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d, Transformed};

use draw::{draw_block, Block, BLOCK_SIZE};

const SNAKE_COLOR: Color = [0.34, 0.80, 0.17, 1.0];
const EYE_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const EYE_SIZE: f64 = (BLOCK_SIZE as f64) * 0.2;
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

struct Point {
    x: f64,
    y: f64,
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

    fn draw_eyes(&self, context: &Context, g2d: &mut G2d) {
        let (left_eye, right_eye) = self.get_eye_positions();
        rectangle(
            EYE_COLOR,
            [left_eye.x, left_eye.y, EYE_SIZE, EYE_SIZE],
            context.transform,
            g2d,
        );
        rectangle(
            EYE_COLOR,
            [right_eye.x, right_eye.y, EYE_SIZE, EYE_SIZE],
            context.transform,
            g2d,
        );
    }

    fn get_eye_positions(&self) -> (Point, Point) {
        let (head_x, head_y): (u32, u32) = self.position();
        let head_corner = Point {
            x: (head_x * BLOCK_SIZE) as f64,
            y: (head_y * BLOCK_SIZE) as f64,
        };
        let center_offset: f64 = (EYE_SIZE as f64) / 2.0;
        let block_offset_third: f64 = (BLOCK_SIZE as f64) / 3.0;
        let block_offset_two_thirds: f64 = block_offset_third * 2.0;
        let (unadjusted_left_eye, unadjusted_right_eye) = match self.direction {
            Direction::Up => (
                Point {
                    x: head_corner.x + block_offset_third,
                    y: head_corner.y + block_offset_third,
                },
                Point {
                    x: head_corner.x + block_offset_two_thirds,
                    y: head_corner.y + block_offset_third,
                },
            ),
            Direction::Down => (
                Point {
                    x: head_corner.x + block_offset_two_thirds,
                    y: head_corner.y + block_offset_two_thirds,
                },
                Point {
                    x: head_corner.x + block_offset_third,
                    y: head_corner.y + block_offset_two_thirds,
                },
            ),
            Direction::Left => (
                Point {
                    x: head_corner.x + block_offset_third,
                    y: head_corner.y + block_offset_two_thirds,
                },
                Point {
                    x: head_corner.x + block_offset_third,
                    y: head_corner.y + block_offset_third,
                },
            ),
            Direction::Right => (
                Point {
                    x: head_corner.x + block_offset_two_thirds,
                    y: head_corner.y + block_offset_third,
                },
                Point {
                    x: head_corner.x + block_offset_two_thirds,
                    y: head_corner.y + block_offset_two_thirds,
                },
            ),
        };
        let adjusted_left_eye = Point {
            x: unadjusted_left_eye.x - center_offset,
            y: unadjusted_left_eye.y - center_offset,
        };
        let adjusted_right_eye = Point {
            x: unadjusted_right_eye.x - center_offset,
            y: unadjusted_right_eye.y - center_offset,
        };
        (adjusted_left_eye, adjusted_right_eye)
    }

    fn draw_tongue(&self, context: &Context, g2d: &mut G2d) {
        let (head_x, head_y): (u32, u32) = self.position();
        let center_x = ((head_x * BLOCK_SIZE) as f64) + ((BLOCK_SIZE as f64) / 2.0);
        let center_y = ((head_y * BLOCK_SIZE) as f64) + ((BLOCK_SIZE as f64) / 2.0);
        let pixel_size = (BLOCK_SIZE as f64) * 0.1;

        rectangle(
            TONGUE_COLOR,
            [
                pixel_size * -0.5,
                ((BLOCK_SIZE as f64) * -0.5) - (pixel_size * 2.0),
                pixel_size,
                (pixel_size * 2.0),
            ],
            context.transform
                .trans(center_x, center_y)
                .rot_rad(match self.direction {
                    Direction::Up => 0.0,
                    Direction::Right => PI * 0.5,
                    Direction::Down => PI,
                    Direction::Left => PI * 1.5,
                }),
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
            context
                .transform
                .trans(center_x, center_y)
                .rot_rad(match self.direction {
                    Direction::Up => 0.0,
                    Direction::Right => PI * 0.5,
                    Direction::Down => PI,
                    Direction::Left => PI * 1.5,
                }),
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
            context
                .transform
                .trans(center_x, center_y)
                .rot_rad(match self.direction {
                    Direction::Up => 0.0,
                    Direction::Right => PI * 0.5,
                    Direction::Down => PI,
                    Direction::Left => PI * 1.5,
                }),
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
