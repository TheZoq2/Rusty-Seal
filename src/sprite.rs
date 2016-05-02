extern crate nalgebra as na;

pub struct Sprite 
{
    transform: na::Matrix3<f32>,
    
    position: na::Vector2<f32>,
    scale: na::Vector2<f32>,
    angle: f32,

    depth: i32,
}

impl Sprite
{
    pub fn new() -> Sprite 
    {
        let mut result = Sprite 
        {
            transform: na::one(),
            position: na::zero(),
            scale: na::one(),
            angle: 0.0,

            depth: 0,
        };

        result.update_transform();
        result
    }

    //Apply the rotation, scale and position to the transform. This should be done automatically when
    //using the set functions
    //
    //The matrix has this layout
    // scale.x*cos(a) ,  scale.x*sin(a), pos.x
    // -scale.y*sin(a),  scale.y*cos(a), pos.y
    // 0              ,  0             ,     1
    fn update_transform(&mut self) 
    {
        self.transform[(0, 2)] = self.position[0];
        self.transform[(1, 2)] = self.position[1];

        self.transform[(0, 0)] = self.scale.x * self.angle.cos();
        self.transform[(0, 1)] = self.scale.y * self.angle.sin();
        self.transform[(1, 0)] = -self.scale.x * self.angle.sin();
        self.transform[(1, 1)] = self.scale.y * self.angle.cos();
    }

    pub fn set_position(&mut self, position: na::Vector2<f32>) 
    {
        self.position = position;
        self.update_transform();
    }
    pub fn set_scale(&mut self, scale: na::Vector2<f32>)
    {
        self.scale = scale;
        self.update_transform();
    }
    pub fn set_angle(&mut self, angle: f32)
    {
        self.angle = angle;
        self.update_transform();
    }

    pub fn get_position(&self) -> na::Vector2<f32>
    {
        return self.position;
    }
    pub fn get_scale(&self) -> na::Vector2<f32>
    {
        return self.scale;
    }
    pub fn get_angle(&self) -> f32
    {
        return self.angle;
    }
    
    pub fn set_depth(&mut self, depth: i32)
    {
        self.depth = depth;
    }
    
    pub fn get_depth(&self) -> i32
    {
        return self.depth;
    }
}

#[cfg(test)]
mod tests
{
    use sprite::*;
    extern  crate nalgebra as na;

    #[test]
    fn transform_test()
    {
        let mut tested_sprite =  Sprite::new();

        assert_eq!(tested_sprite.position, tested_sprite.get_position());
        assert_eq!(tested_sprite.angle, tested_sprite.get_angle());
        assert_eq!(tested_sprite.scale, tested_sprite.get_scale());
        assert_eq!(tested_sprite.transform, na::one());

        //Test scale changes
        
        tested_sprite.set_scale(na::Vector2::new(2.0, 2.0));
        assert_eq!(na::Matrix3::<f32>::new(2.0,0.0,0.0, 0.0,2.0,0.0, 0.0,0.0,1.0), tested_sprite.transform);

        tested_sprite.set_scale(na::Vector2::new(2.0, 0.5));
        assert_eq!(na::Matrix3::<f32>::new(2.0,0.0,0.0, 0.0,0.5,0.0, 0.0,0.0,1.0), tested_sprite.transform);

        //Reset the scale and test positions
        tested_sprite.set_scale(na::Vector2::new(1.0, 1.0));
        tested_sprite.set_position(na::Vector2::new(3.0, 5.0));
        assert_eq!(na::Matrix3::<f32>::new(1.0,0.0,3.0, 0.0,1.0,5.0, 0.0,0.0,1.0), tested_sprite.transform);

        //Reset positon, test rotation
        tested_sprite.set_position(na::Vector2::new(0.0, 0.0));
        let angle: f32 = 1.3;
        tested_sprite.set_angle(1.3);
        assert_eq!(na::Matrix3::new(angle.cos(),angle.sin(),0.0, -angle.sin(),angle.cos(),0.0, 0.0,0.0,1.0), tested_sprite.transform);

        let scale = na::Vector2::new(5.0, 3.0);
        let position = na::Vector2::new(10.0, -10.0);
        let mut test_vector = na::Vector3::new(1.0, 1.0, 1.0);
        
        tested_sprite.set_angle(angle);
        tested_sprite.set_position(position);
        tested_sprite.set_scale(scale);

        println!("{}", tested_sprite.transform);
    
        test_vector = tested_sprite.transform * test_vector;
        assert_eq!(na::Vector3::new(
                        scale.x*angle.cos() + scale.y*angle.sin() + position.x,
                        -scale.x*angle.sin() + scale.y * angle.cos() + position.y,
                        1.0
                    ), test_vector);
    }
}
