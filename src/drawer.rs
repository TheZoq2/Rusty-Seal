extern crate nalgebra as na;
 
use glium;
use glium::Surface;


use gl_structs::*;

use sprite::*;

use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

const SPRITE_VERTICES: [Vertex;4] = [
    Vertex{position: (0.0,0.0,1.0)}, 
    Vertex{position: (0.0,1.0,1.0)},
    Vertex{position: (1.0,1.0,1.0)}, 
    Vertex{position: (1.0,0.0,1.0)}
];
const SPRITE_NORMALS: [Normal;4] = [
    Normal{normal: (1.0,1.0,0.0)}, 
    Normal{normal: (1.0,1.0,0.0)}, 
    Normal{normal: (1.0,1.0,0.0)}, 
    Normal{normal: (1.0,1.0,0.0)}
];
const SPRITE_INDICES: [u16; 4] = [
    0,
    1,
    2,
    3,
];



//TODO: error handling
pub fn load_whole_file(path: &String) -> String
{
    let file_path = Path::new(&path);

    let mut file = match File::open(&file_path){
        Err(why) => panic!("Failed to open file {}, {}", &path, Error::description(&why)),

        Ok(open_file) => open_file
    };

    let mut result = String::new();
    match file.read_to_string(&mut result) {
        Err(why) => panic!("Failed to read content of file {}, {}", &path, Error::description(&why)),
        Ok(a) => a
    };

    result
}

//TODO: Error handling
//TODO: MOve somewhere else
pub fn load_shader(display: &glium::Display, vert_path: &String, frag_path: &String) -> glium::Program 
{
    let vert_src = load_whole_file(vert_path);
    let frag_src = load_whole_file(frag_path);

    glium::Program::from_source(display, &vert_src, &frag_src, None).unwrap()
}

pub struct Drawer<'a>
{
    display: &'a glium::Display,
    draw_surface: glium::Frame,

    draw_params: glium::DrawParameters<'a>,

    vertecies: glium::VertexBuffer<Vertex>,
    normals: glium::VertexBuffer<Normal>,
    indices: glium::IndexBuffer<u16>,
}

impl<'a> Drawer<'a>
{
    pub fn new(display: &'a glium::Display) -> Drawer<'a>
    {
        //Create vertecies before moving the display object into the struct
        let vertecies = glium::VertexBuffer::<Vertex>::new(display, &SPRITE_VERTICES).unwrap();
        let normals = glium::VertexBuffer::<Normal>::new(display, &SPRITE_NORMALS).unwrap();
        let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TriangleStrip, &SPRITE_INDICES).unwrap();

        //Draw parameters for the rendering
        let draw_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        //Getting the surface to draw on
        let draw_surface = display.draw();

        Drawer {
            display: display,
            draw_surface: draw_surface,

            vertecies: vertecies,
            normals: normals,
            indices: indices,

            draw_params: draw_params,
        }
    }


    pub fn clear(&mut self)
    {
        self.draw_surface = self.display.draw();

        self.draw_surface.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
    }
    //Runs the finish function on the drawing surface and destroys this instance
    pub fn finish(self)
    {
        self.draw_surface.finish().unwrap();
    }

    pub fn draw_sprite(&mut self, sprite: Sprite) 
    {
        //Create the uniform for the sprite
        let uniforms = uniform!{
            transform: sprite.get_transform().as_ref().clone(),
            texture: sprite.get_texture(),
        };

        
        self.draw_surface.draw(
                (&self.vertecies, &self.normals), 
                &self.indices, 
                sprite.get_shader(), 
                &uniforms, 
                &self.draw_params
            ).unwrap();
    }
}
