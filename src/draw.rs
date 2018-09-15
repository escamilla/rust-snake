use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

pub const BLOCK_SIZE: u32 = 30;

#[derive(Clone, Debug)]
pub struct Block {
    pub x: u32,
    pub y: u32,
}

pub fn draw_rectangle(
    color: Color,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    context: &Context,
    g2d: &mut G2d,
) {
    rectangle(
        color,
        [
            (x * BLOCK_SIZE) as f64,
            (y * BLOCK_SIZE) as f64,
            (width * BLOCK_SIZE) as f64,
            (height * BLOCK_SIZE) as f64,
        ],
        context.transform,
        g2d,
    );
}

pub fn draw_block(color: Color, x: u32, y: u32, context: &Context, g2d: &mut G2d) {
    draw_rectangle(color, x, y, 1, 1, context, g2d);
}
