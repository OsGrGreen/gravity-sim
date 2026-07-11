use crate::{scene::objects::WorldObject, util::input_handler::InputHandler};

pub mod gravity_controller;

pub mod path_controller;

pub mod movement_controller;


pub trait Controller {
    fn update(&mut self, objects: &mut Vec<WorldObject>, input: &InputHandler);
    fn add(&mut self, objects: Vec<&WorldObject>);
    fn add_single(&mut self, object: &WorldObject);
}