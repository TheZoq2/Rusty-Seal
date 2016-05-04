extern crate nalgebra as na;
 
use glium;


use gl_structs::*;
use glium::texture::texture2d::*;

use sprite::*;

const SPRITE_INDICES: [Vertex;4] = [
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

pub struct Drawer
{
    display: glium::Display,

    vertecies: glium::VertexBuffer<Vertex>,
    normals: glium::VertexBuffer<Normal>,
}

impl Drawer
{
    pub fn new(display: glium::Display) -> Drawer 
    {
        //Create vertecies before moving the display object into the struct
        let vertecies = glium::VertexBuffer::<Vertex>::new(&display, &SPRITE_INDICES).unwrap();
        let normals = glium::VertexBuffer::<Normal>::new(&display, &SPRITE_NORMALS).unwrap();

        Drawer {
            display: display,

            vertecies: vertecies,
            normals: normals,
        }
    }


    pub fn draw_sprite(sprite: Sprite) 
    {
        
    }
}
