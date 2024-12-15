use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

//pub = public
//takes in game coordinate and returns an f64 of that coordinate * block size
pub fn to_coord(game_coord: i32) -> f64 {
    //multiplies our gamecoord by the block size
    (game_coord as f64) * BLOCK_SIZE
}

//same thing except returns u32
pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

//draws a block at an x and a y
//passes in a color, x, y, context, mutable graphics buffer
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    //converts x and y to coordinates based on block size
    let gui_x = to_coord(x);
    let gui_y: f64 = to_coord(y);

    //calls rectangle and passes in color, width and height, con transform, and graphics buffer
    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

//draws a rectangle of blocks
//same as draw_block except also passing in a width and height
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
