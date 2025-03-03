use super::PhysicsObject;

pub struct Sphere{
    radius: f32,
}

impl PhysicsObject for Sphere{
    fn init(&mut self) {
        todo!()
    }
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Sphere { radius }
    }
}