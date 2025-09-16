
use super::Controller;

pub struct Gravity{
    G: f32,
}

impl Gravity{
    pub fn new(g_const: f32) -> Gravity{
        Gravity { G: g_const }
    }
}

impl Controller for Gravity{
    fn update(&mut self, objects: &mut Vec<crate::scene::objects::WorldObject>) {
        for i in 0..objects.len() {
            let (before, after) = objects.split_at_mut(i);
            let (obj1, after) = after.split_first_mut().unwrap(); // Get obj1 from after
                for obj2 in before.iter().chain(after.iter()) {
                    let (dir, distance) = obj1.distance(obj2);
                    let force = self.G*(obj1.physics_object.get_mass()*obj2.physics_object.get_mass())/(distance*distance);
                    obj1.physics_object.add_force(dir.normalize()*force);
                    //Collision
                    if obj1.collides(obj2){
                        //println!("Collision between {:?} and {:?}", obj1,obj2);
                    }
                }
        }
    }
}