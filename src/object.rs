type Point = (i32, i32);

pub struct Object{
    position: Point,
    speed: int,
    direction: int,
    mass: int,
}

impl Object{

    fn new() -> Self{
        Object{
            position: (10,10),
            speed: 0,
            direction: 0,
            mass: 0
        }
    }
}