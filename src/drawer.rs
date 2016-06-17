extern crate nalgebra as na;
 
use glium;
use glium::Surface;


use gl_structs::*;
use glium::texture::texture2d::*;

use sprite::*;

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

pub struct Drawer<'a>
{
    display: glium::Display,
    draw_surface: glium::Frame,

    draw_params: glium::DrawParameters<'a>,
    
    //sprite_shader: glium::Program,

    vertecies: glium::VertexBuffer<Vertex>,
    normals: glium::VertexBuffer<Normal>,
    indices: glium::IndexBuffer<u16>,
}

impl<'a> Drawer<'a>
{
    pub fn new(display: glium::Display) -> Drawer<'a>
    {
        //Create vertecies before moving the display object into the struct
        let vertecies = glium::VertexBuffer::<Vertex>::new(&display, &SPRITE_VERTICES).unwrap();
        let normals = glium::VertexBuffer::<Normal>::new(&display, &SPRITE_NORMALS).unwrap();
        let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TriangleStrip, &SPRITE_INDICES).unwrap();

        let draw_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

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
    pub fn finish(mut self)
    {
        self.draw_surface.finish().unwrap();
    }

    pub fn draw_sprite(&mut self, sprite: Sprite) 
    {
        //Create the uniform for the sprite
        let uniforms = uniform!{
            //transform: sprite.get_transform(),
        };

        //self.target.draw((&self.vertices, &self.normals), &self.indices, &self.uniforms, &self.draw_params).unwrap();
    }
}
