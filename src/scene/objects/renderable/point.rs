use glam::{Vec2, Vec3};

use crate::scene::objects::transform::Transform;

#[derive(Copy, Clone,Debug)]

pub struct WorldPoint{
    obj: Transform,
    radius: f32,
    center: Vec2,
}

impl WorldPoint{
    pub fn new(radius: f32, center: Vec2, pos: Vec3) -> WorldPoint{
        WorldPoint{
            obj: Transform::new_from_pos(pos),
            radius: radius,
            center: center,
        }
    }

    pub fn get_model(self) -> Transform{
        self.obj
    }
    
    pub fn get_radius(self) -> f32{
        self.radius
    }

    pub fn get_center(self) -> Vec2{
        self.center
    }

    pub fn get_mut_model(&mut self) -> &mut Transform{
        &mut self.obj
    }
}