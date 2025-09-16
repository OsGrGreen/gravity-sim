use crate::scene::objects::WorldObject;

pub mod gravity_controller;

pub mod path_controller;


pub trait Controller {
    fn update(&mut self, objects: &mut Vec<WorldObject>);
}