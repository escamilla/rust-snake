use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: u32 = 25;

#[derive(Clone, Debug)]
pub struct Block {
    pub x: u32,
    pub y: u32,
}

pub fn convert_coord_to_pixels(coord: u32) -> u32 {
    coord * BLOCK_SIZE
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
            convert_coord_to_pixels(x) as f64,
            convert_coord_to_pixels(y) as f64,
            convert_coord_to_pixels(width) as f64,
            convert_coord_to_pixels(height) as f64,
        ],
        context.transform,
        g2d,
    );
}

pub fn draw_block(color: Color, x: u32, y: u32, context: &Context, g2d: &mut G2d) {
    draw_rectangle(color, x, y, 1, 1, context, g2d);
}
