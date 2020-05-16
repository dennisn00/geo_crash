use piston_window::*;

type Point = (i32, i32);

pub struct GameObject {
    position: Point,
    speed: u32,
    direction: u32,
    mass: u32,
}

impl GameObject {

    fn new() -> Self{
        GameObject {
            position: (10,10),
            speed: 0,
            direction: 0,
            mass: 0
        }
    }

    fn render(&mut self, context: Context, graphics: G2d){
        //TODO: render Object at current position
    }
}