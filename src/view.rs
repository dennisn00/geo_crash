use piston_window::*;
use piston_window::ellipse;
use crate::master::Master;
use piston_window::color;
use piston_window::ellipse::circle;

pub struct View {
}

impl View{

    pub fn new() -> Self{
        View{
        }
    }
    pub fn render(&mut self, context: Context, graphics: &mut G2d, master: &Master){
        //get width and height of window from context
        let [width, height] = context.get_view_size();

        //clear the screen
        clear([0.5, 0.5, 0.5, 1.0], graphics);

        //TODO: this should call the render functions of all objects in master
        //for demonstration and testing purposes, we draw objects here now
        //draw circle
        ellipse(color::WHITE, circle(width/2f64, height/2f64, 100f64), context.transform, graphics)

    }

    pub fn destroy(self){

    }
}