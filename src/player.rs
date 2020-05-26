use graphics::{Context, Drawable};
use ggez::Context;

pub struct Player{
    //TODO: implement player attributes
}

impl Drawable for Player{

    fn draw(&self, ctx: &mut Context, param: DrawParam){

    }

    fn dimensions(&self, ctx: &mut Context){

    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>){

    }

    fn blend_mode(&self){

    }
}

impl Player{

    pub fn new() -> Self {
        //TODO: create a new player in the center of the screen
        Player{}
    }

    pub fn update(&mut self, context: Context, graphics: G2d){
        //TODO: render player
    }

    pub fn draw(&self, context: &Context){

    }
}