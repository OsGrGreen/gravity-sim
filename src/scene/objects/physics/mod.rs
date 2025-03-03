use sphere::Sphere;

pub mod sphere;

pub trait PhysicsObject{
    fn init(&mut self);
}


pub fn physics_object_factory(id: usize) -> Box<dyn PhysicsObject> {
    match id {
        0 => Box::new(Sphere::new(2.0)), 
        _ => panic!("Unknown shape"),
    }
}