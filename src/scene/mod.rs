use std::{collections::HashMap, hash::Hash};

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use glium::{framebuffer::SimpleFrameBuffer, glutin::surface::WindowSurface, texture::DepthTexture2d, Display, Surface, Texture2d};
use light::Light;
use objects::{physics::{physics_object_factory, PhysicsObject}, renderable::renderobjects::RenderObject, WorldObject};
use rand::Rng;
use winit::{keyboard, window::Window};

use crate::{rendering::{render::{self, Renderer}, render_camera::RenderCamera}, scene::objects::physics::controllers::{self, gravity_controller::Gravity, Controller}, spline::Spline, util::{create_fbo, create_render_textures, input_handler::{self, InputHandler}, load_texture}};

pub mod bezier_surface;
pub mod objects;
pub mod light;

pub struct Scene<'a>{
    world_objects: Vec<WorldObject>,
    lights: Vec<Light>,
    renderers: HashMap<String,Renderer<'a>>,
    textures: HashMap<String,Texture2d>,
    controllers: Vec<Box<dyn Controller>>,
    pub camera: RenderCamera,
    pub scene_tex: Texture2d,
    scene_depth: DepthTexture2d,
}

impl<'a> Scene<'a>{
    pub fn new(camera: RenderCamera, lights: Option<Vec<Light>>, display: &Display<WindowSurface>, size: (u32,u32)) -> Scene<'a>{
        let (world_texture, depth_world_texture) = create_render_textures(&display,size.0, size.1);
        let gravity = Gravity::new(6.67e-5);
        Scene { world_objects: Vec::new(), lights: lights.unwrap_or(Vec::new()), renderers: HashMap::new(), textures: HashMap::new(), controllers: vec![Box::new(gravity)], camera, scene_tex: world_texture, scene_depth: depth_world_texture}
    }

    pub fn draw(&mut self, display: &Display<WindowSurface>){
        let mut fbo = create_fbo(&display, &self.scene_tex, &self.scene_depth);
        fbo.clear_color_and_depth((0.05, 0.05, 0.14, 1.0), 1.0);
        for render in &mut self.world_objects{
            let render_id = &render.render_object.render_id;
            if render_id.is_some(){
                render.draw(&mut fbo, &self.camera, self.renderers.get(render_id.as_ref().unwrap().as_str()).unwrap(), &mut self.textures.get(&render.id));
            }
            
        }
        let mut spline = Spline::new_empty();
        spline.insert([Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 2.1), Vec3::new(1.0, 2.0, 0.0), Vec3::new(1.0, 0.0, -0.5)]);
        spline.insert_c2(Vec3::new(-2.0, 0.0, 0.0));
        //spline.insert_c2(Vec3::new(0.0, 1.0, 0.5));
        //spline.insert_c2(Vec3::new(2.0,0.0,0.0));
        //spline.insert_c0([Vec3::new(2.0, -0.5, 0.5), Vec3::new(0.0, -1.0, 1.0), Vec3::new(3.0, -2.0, 2.0)]);
        //spline.insert_c0([Vec3::new(2.0, -4.0, 2.0),Vec3::new(0.5, 5.0, 2.0),Vec3::new(0.5, 0.0, 0.0)]);

        let (vbo, inds, rend) = spline.spline_renderer(display);
        //println!("VBO {:?}", vbo);
        fbo.draw(&vbo, &inds, &rend.program, &uniform! {u_screenSize: [self.scene_tex.dimensions().0 as f32, self.scene_tex.dimensions().1 as f32], u_thickness: 50.0 as f32, steps: 48.0 as f32, model: Mat4::IDENTITY.to_cols_array_2d(), projection: self.camera.perspective.to_cols_array_2d(), view:self.camera.getMatrix()}, &rend.draw_params).unwrap();
    }

    pub fn add_generic_renderer(&mut self, name: &str, display: &Display<WindowSurface>,vertex_data: &[u8], fragment_data: &[u8], obj_data: &[u8]){
        self.renderers.insert(name.to_string(), Renderer::init(display, vertex_data, fragment_data, obj_data).unwrap());
    }

    pub fn get_renderer(&self, name: &str) -> Option<&Renderer>{
        self.renderers.get(name)
    }

    pub fn new_object(&mut self, object_name: &str, render_name: &str, display: &Display<WindowSurface>,vertex_data: &[u8], fragment_data: &[u8], obj_data: &[u8]) -> usize{
        self.renderers.insert(render_name.to_string(), Renderer::init(display, vertex_data, fragment_data, obj_data).unwrap());
        let obj = WorldObject::new(object_name.to_string(),RenderObject::new(Some(render_name.to_string())), physics_object_factory(0, vec![1.0,1.0]));
        self.world_objects.push(obj);
        return self.world_objects.len() - 1;
    }
    

    pub fn new_object_instance(&mut self, object_name: &str,render_name: &str) -> usize{
        let obj = WorldObject::new(object_name.to_string(),RenderObject::new(Some(render_name.to_string())), physics_object_factory(0, vec![1.0,1.0]));
        self.world_objects.push(obj);
        return self.world_objects.len() - 1;
    }
    
    pub fn add_object(&mut self, obj: WorldObject) -> usize{
        let index = self.world_objects.len();
        self.world_objects.push(obj);
        return index
    }

    pub fn add_texture(&mut self, object_name: &str, display: &Display<WindowSurface>, tex_data: &[u8]){
        self.textures.insert(object_name.to_string(), load_texture(display, tex_data));
    }

    pub fn update_camera(&mut self, dt: f32, input_handler: &InputHandler){
        //println!("mouse pos: {}", input_handler.pos());
        if input_handler.is_pressed(keyboard::KeyCode::ControlLeft){
            let mut yaw = 0.0;
            let mut pitch = 0.0;
            if input_handler.pos().x.abs() > 0.2{
                yaw = 5.0*dt*input_handler.pos().x;
            }
            if input_handler.pos().y.abs() > 0.3{
                //println!("Right angle is {}", self.camera.get_right());
                pitch =  5.0*dt*input_handler.pos().y;
            }
            self.camera.update(pitch, yaw);
        }
    }

    pub fn update_physics(&mut self, dt: f32){
        self.controllers[0].as_mut().update(&mut self.world_objects);
        for obj in &mut self.world_objects{
            obj.update_physics(dt);
        }


    }

    pub fn init_gravity_scene(window: &Window, display: &Display<WindowSurface>, size: (u32,u32)) -> Scene<'a>{
        let planet_renderer  = Renderer::init(display, include_bytes!(r"../../shaders/planet/vert.glsl"), include_bytes!(r"../../shaders/planet/frag.glsl"), include_bytes!(r"../../objects/planet.obj")).unwrap();
        let mut render_map = HashMap::new();
        render_map.insert("planet".to_string(), planet_renderer);
        
        let mut sun = WorldObject::new("sun".to_string(), RenderObject::new(Some("planet".to_string())), physics_object_factory(0, vec![1.0,50000.0]));
        sun.render_object.model_object.translate(Vec3::new(0.0, 0.0, 0.0));
        
        let mut planet1 = WorldObject::new("planet1".to_string(),RenderObject::new(Some("planet".to_string())), physics_object_factory(0, vec![0.5,2.0]));
        planet1.render_object.model_object.translate(Vec3::new(-5.0, -3.0, 2.0));
        planet1.physics_object.set_init_velocity(Vec3::new(0.0, 3.0, 1.0));
        
        let mut planet2 = WorldObject::new("planet2".to_string(),RenderObject::new(Some("planet".to_string())), physics_object_factory(0, vec![0.5,1.0]));
        planet2.render_object.model_object.translate(Vec3::new(-7.0, 3.0, 0.0));
        planet2.physics_object.set_init_velocity(Vec3::new(1.0, 4.0, 0.0));
        
        let mut planet3 = WorldObject::new("planet3".to_string(),RenderObject::new(Some("planet".to_string())), physics_object_factory(0, vec![0.75,5.0]));
        planet3.render_object.model_object.translate(Vec3::new(5.0, -4.0, 0.0));
        planet3.physics_object.set_init_velocity(Vec3::new(1.5, 2.0, 0.0));

        let mut solar_system = vec![sun, planet1, planet2, planet3];

        let mut rng = rand::rng();
        for _ in 0..0{
            let mut add_planet = WorldObject::new("planet3".to_string(),RenderObject::new(Some("planet".to_string())), physics_object_factory(0, vec![rng.random_range(0.0..1.5),rng.random_range(0.1..20.0)]));
            add_planet.render_object.model_object.translate(Vec3::new(rng.random_range(-5.5..5.5), rng.random_range(-5.5..5.5), 0.0));
            add_planet.physics_object.set_init_velocity(Vec3::new(rng.random_range(-1.5..1.5), rng.random_range(-2.0..2.0), 0.0));
            solar_system.push(add_planet);
        }

        let (world_texture, depth_world_texture) = create_render_textures(&display,size.0, size.1);
        let gravity = Gravity::new(6.67e-5);
        let mut return_scene = Scene { world_objects: solar_system, lights: Vec::new(), renderers: render_map, controllers: vec![Box::new(gravity)], camera: RenderCamera::new(Vec3::new(0.0, 0.0, 15.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0), window.inner_size().into()), textures: HashMap::new(), scene_tex: world_texture, scene_depth: depth_world_texture};
        
        return_scene.add_texture("sun", display, include_bytes!(r"../../textures/sun.png"));
        return_scene.add_texture("planet1", display, include_bytes!(r"../../textures/planet.png"));
        
        return return_scene;
    }
}