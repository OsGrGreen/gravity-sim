use std::{collections::HashMap, hash::Hash};

use glam::Vec3;
use glium::{framebuffer::SimpleFrameBuffer, glutin::surface::WindowSurface, Display};
use light::Light;
use objects::{physics::{physics_object_factory, PhysicsObject}, renderable::renderobjects::RenderObject, WorldObject};

use crate::{rendering::{render::{self, Renderer}, render_camera::RenderCamera}, util::input_handler::InputHandler};

pub mod bezier_surface;
pub mod objects;
pub mod light;

pub struct Scene<'a>{
    world_objects: Vec<WorldObject>,
    lights: Vec<Light>,
    renderers: HashMap<String,Renderer<'a>>,
    pub camera: RenderCamera
}

impl<'a> Scene<'a>{
    pub fn new(camera: RenderCamera, lights: Option<Vec<Light>>) -> Scene<'a>{
        Scene { world_objects: Vec::new(), lights: lights.unwrap_or(Vec::new()), renderers: HashMap::new(),camera: camera }
    }

    pub fn draw(&mut self, fbo: &mut SimpleFrameBuffer<'_>){
        for render in &mut self.world_objects{
            let render_id = &render.render_object.render_id;
            if render_id.is_some(){
                render.draw(fbo, &self.camera, self.renderers.get(render_id.as_ref().unwrap().as_str()).unwrap());
            }
            
        }
    }

    pub fn add_generic_renderer(&mut self, name: &str, display: &Display<WindowSurface>,vertex_data: &[u8], fragment_data: &[u8], obj_data: &[u8]){
        self.renderers.insert(name.to_string(), Renderer::init(display, vertex_data, fragment_data, obj_data).unwrap());
    }

    pub fn get_renderer(&self, name: &str) -> Option<&Renderer>{
        self.renderers.get(name)
    }

    pub fn new_object(&mut self, render_name: &str, display: &Display<WindowSurface>,vertex_data: &[u8], fragment_data: &[u8], obj_data: &[u8]) -> usize{
        self.renderers.insert(render_name.to_string(), Renderer::init(display, vertex_data, fragment_data, obj_data).unwrap());
        let obj = WorldObject::new(RenderObject::new(Some(render_name.to_string())), physics_object_factory(0));
        self.world_objects.push(obj);
        return self.world_objects.len() - 1;
    }
    

    pub fn new_object_instance(&mut self, render_name: &str) -> usize{
        let obj = WorldObject::new(RenderObject::new(Some(render_name.to_string())), physics_object_factory(0));
        self.world_objects.push(obj);
        return self.world_objects.len() - 1;
    }
    
    pub fn add_object(&mut self, obj: WorldObject) -> usize{
        let index = self.world_objects.len();
        self.world_objects.push(obj);
        return index
    }

    pub fn update_camera(&mut self, input: &InputHandler){}

    pub fn update_physics(&mut self, dt: f32){
        for obj in &mut self.world_objects{
            obj.render_object.model_object.rotate(Vec3::new(0.0, 1.0, 0.0), dt*1.0);
            obj.render_object.model_object.translate(dt*Vec3::new(0.0, 0.0, -1.0));
        }
    }
}