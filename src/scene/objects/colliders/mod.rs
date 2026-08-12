use glam::Vec3;

pub struct Collider {
    pub shape: CollisionShape,
}

pub enum CollisionShape {
    Sphere { radius: f32 },
    Box { half_extents: Vec3 },
    Capsule { radius: f32, height: f32 },
}

impl Collider {
    pub fn new(shape: CollisionShape) -> Collider {
        Collider { shape }
    }

    pub fn collides(&self, other: &Option<Collider>) -> bool {
        return false;
    }
}