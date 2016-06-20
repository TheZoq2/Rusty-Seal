#![allow(dead_code)]
#[macro_use]

extern crate glium;
extern crate nalgebra as na;
extern crate image;

mod sprite;
mod drawer;
mod gl_structs;

use glium::DisplayBuild;
use glium::Surface;

use std::io::Cursor;
use std::rc::Rc;

pub fn main() 
{
    let display = glium::glutin::WindowBuilder::new()
                    .with_depth_buffer(24)
                    .build_glium()
                    .unwrap();

    //Loading a test image
    let img = image::load(Cursor::new(&include_bytes!("../data/img/test.png")[..]),
                image::PNG).unwrap().to_rgba();

    let img_dimensions = img.dimensions();
    let img = glium::texture::RawImage2d::from_raw_rgba_reversed(img.into_raw(), img_dimensions);


    let test_texture = Rc::new(glium::texture::Texture2d::new(&display, img).unwrap());

    //Loading a test shader
    let shader = Rc::new(drawer::load_shader(&display, 
                                             &String::from("data/shaders/basic.vert"), 
                                             &String::from("data/shaders/basic.frag")));
    
    //Main engine loop
    loop
    {
        //Create a drawer to draw the current frame
        let mut main_drawer = drawer::Drawer::new(&display);
        


        //Finish the drawing process destroying the drawer
        main_drawer.finish();

        //Handling events
        for ev in display.poll_events()
        {
            match ev
            {
                glium::glutin::Event::Closed => return,
    
                _ => ()
            }
        }
    }
}
