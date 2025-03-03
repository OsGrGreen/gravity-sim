use glam::{Mat4, Vec2, Vec3};

use super::ModelObject;
#[derive(Copy, Clone,Debug)]

pub struct WorldPoint{
    obj: ModelObject,
    radius: f32,
    center: Vec2,
}

impl WorldPoint{
    pub fn new(radius: f32, center: Vec2, pos: Vec3) -> WorldPoint{
        WorldPoint{
            obj: ModelObject::new_from_pos(pos),
            radius: radius,
            center: center,
        }
    }

    pub fn get_model(self) -> ModelObject{
        self.obj
    }
    
    pub fn get_radius(self) -> f32{
        self.radius
    }

    pub fn get_center(self) -> Vec2{
        self.center
    }

    pub fn get_mut_model(&mut self) -> &mut ModelObject{
        &mut self.obj
    }
}