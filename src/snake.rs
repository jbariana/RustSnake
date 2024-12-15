use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

//bringing draw_block function in from draw.rs
use crate::draw::draw_block;

//rgbo
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

//directions
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    //takes in a reference to self and outputs a direction
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

//block objects have an x and a y value denoting position
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

//implements snake so we can have methods
impl Snake {
    //pass in an x and y and return a snake object
    pub fn new(x: i32, y: i32) -> Snake {
        //creates a new body for the snake which is a linkedlist of blocks
        let mut body: LinkedList<Block> = LinkedList::new();
        //appending the 3 initial blocks of our snaketo the linked list
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });
        //default direction right, body is default, tail is none by default
        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    //takes in a reference to self, context, and g2d
    //loops through all blocks in the snake and draws them with draw_block
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    //takes in a mutable self, outputs a tuple of i32s
    //.front returns the front element of the linkedlist
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    //takes in self and a direction
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            //snake direction is d
            Some(d) => self.direction = d,
            //otw pass back none
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        //makes a new block in the direction that the snake is going
        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
        //push the new block on to the front
        self.body.push_front(new_block);
        //remove the last block
        let removed_block = self.body.pop_back().unwrap();
        //tail is now the removed block
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    //takes in self and option of directions, outputs tuple of i32s
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        //gets head position
        let (head_x, head_y): (i32, i32) = self.head_position();

        //gets the direction we're moving in
        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        //match on the moving direction and return back these coords
        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    //takes in self and creates a block based on the tail and pushes cloned tail to the back of the body
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    //takes in self and an x and y, returns bool
    //if if our snake is overlapping any part of its body, return true, otw increment ch, if ch = length-1, break
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}
