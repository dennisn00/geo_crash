extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use piston_window::*;
use geoCrash::App;

static TITLE:&str = "GeoCrash";

fn main(){

    //create instance of App
    let mut app = App::new();


    //create GameWindow
    let mut window: PistonWindow = WindowSettings::new(TITLE,
                                                       [512;2]).build().unwrap();


    while let Some(e) = window.next(){

        //if event is render, closure f is called, context has information about window size
        //graphics represents visual state of window
        window.draw_2d(&e, |context, graphics, _| {
            app.view().render(context, graphics, &app.master);
        });
    }
}
