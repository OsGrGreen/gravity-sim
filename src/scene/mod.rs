use core::f32;
use std::{collections::HashMap, hash::Hash, println};

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use glium::{framebuffer::SimpleFrameBuffer, glutin::surface::WindowSurface, texture::DepthTexture2d, Display, Surface, Texture2d};
use light::Light;
use objects::{physics::{PhysicsObject}, renderable::RenderObject, WorldObject};
use rand::Rng;
use winit::{event::MouseButton, keyboard, window::Window};

use crate::{rendering::{render::{self, Renderer}, render_camera::RenderCamera}, scene::objects::{ObjectId, physics::{bodies::{RigidBody, StaticBody}, controllers::{self, Controller, gravity_controller::Gravity, movement_controller::{Movement, PlayerMover}, path_controller::{self, Path}}}, transform::Transform}, spline::Spline, util::{create_fbo, create_render_textures, input_handler::{self, InputHandler}, load_texture}};

pub mod bezier_surface;
pub mod objects;
pub mod light;
pub mod slingshot_scene;

pub struct SceneContent {
    time: f32,
    world_objects: Vec<WorldObject>,
    lights: Vec<Light>,
    pub camera: RenderCamera,
}
pub struct Scene<'a>{
    content: SceneContent,
    render_counter: usize,
    renderers: HashMap<String,Renderer<'a>>,
    textures: HashMap<ObjectId,Texture2d>,
    controllers: Vec<Box<dyn Controller>>,
    pub scene_tex: Texture2d,
    scene_depth: DepthTexture2d,
}

impl<'a> Scene<'a>{
    pub fn new(camera: RenderCamera, lights: Option<Vec<Light>>, display: &Display<WindowSurface>, size: (u32,u32)) -> Scene<'a>{
        let (world_texture, depth_world_texture) = create_render_textures(&display,size.0, size.1);
        let gravity = Gravity::new(6.67e-5);
        Scene { content: SceneContent { time: 0.0, world_objects: Vec::new(), lights: lights.unwrap_or(Vec::new()), camera}, render_counter: 0 , renderers: HashMap::new(), textures: HashMap::new(), controllers: vec![Box::new(gravity)], scene_tex: world_texture, scene_depth: depth_world_texture}
    }

    pub fn draw(&mut self, display: &Display<WindowSurface>){
        let mut fbo = create_fbo(&display, &self.scene_tex, &self.scene_depth);
        fbo.clear_color_and_depth((0.05, 0.05, 0.14, 1.0), 1.0);
        for render in &mut self.content.world_objects{
            let render_id = &render.render.render_id;
            println!("Trying to draw: {}", render.id.index);
            if render_id.is_some(){
                render.draw(&mut fbo, &self.content.camera, self.renderers.get(render_id.as_ref().unwrap().as_str()).unwrap(), &mut self.textures.get(&render.id), self.content.time);
            }
            
        }
        let mut spline = Spline::new_empty();
        spline.insert([self.content.world_objects[0].transform.position, self.content.world_objects[0].transform.position, self.content.world_objects[1].transform.position, self.content.world_objects[1].transform.position]);
        //spline.insert_c2(Vec3::new(-2.0, 0.0, 0.0));
        //spline.insert_c2(Vec3::new(0.0, 1.0, 0.5));
        //spline.insert_c2(Vec3::new(2.0,0.0,0.0));
        //spline.insert_c0([Vec3::new(2.0, -0.5, 0.5), Vec3::new(0.0, -1.0, 1.0), Vec3::new(3.0, -2.0, 2.0)]);
        //spline.insert_c0([Vec3::new(2.0, -4.0, 2.0),Vec3::new(0.5, 5.0, 2.0),Vec3::new(0.5, 0.0, 0.0)]);

       // let loop_spline = Spline::new_circle(Vec3::ZERO, 2.0, 0.0, 0.0, 0.0);

        let (vbo, inds, rend) = spline.spline_renderer(display);
        //println!("VBO {:?}", vbo);
        fbo.draw(&vbo, &inds, &rend.program, &uniform! {u_screenSize: [self.scene_tex.dimensions().0 as f32, self.scene_tex.dimensions().1 as f32], u_thickness: 50.0 as f32, steps: 48.0 as f32, model: Mat4::IDENTITY.to_cols_array_2d(), projection: self.content.camera.perspective.to_cols_array_2d(), view:self.content.camera.getMatrix()}, &rend.draw_params).unwrap();

        //let (vbo2, inds2, rend2) = loop_spline.spline_renderer(display);
        //println!("VBO {:?}", vbo);
        //fbo.draw(&vbo2, &inds2, &rend2.program, &uniform! {u_screenSize: [self.scene_tex.dimensions().0 as f32, self.scene_tex.dimensions().1 as f32], u_thickness: 50.0 as f32, steps: 48.0 as f32, model: Mat4::IDENTITY.to_cols_array_2d(), projection: self.camera.perspective.to_cols_array_2d(), view:self.camera.getMatrix()}, &rend2.draw_params).unwrap();
    }

    pub fn add_generic_renderer(&mut self, name: &str, display: &Display<WindowSurface>,vertex_data: &[u8], fragment_data: &[u8], obj_data: &[u8]){
        self.render_counter += 1;
        self.renderers.insert(name.to_string(), Renderer::init(display, vertex_data, fragment_data, obj_data, None).unwrap());
    }

    pub fn get_renderer(&self, name: &str) -> Option<&Renderer>{
        self.renderers.get(name)
    }

    pub fn new_object(&mut self, object_name: &str, render_name: &str, display: &Display<WindowSurface>,vertex_data: &[u8], fragment_data: &[u8], obj_data: &[u8]) -> usize{
        if self.renderers.insert(render_name.to_string(), Renderer::init(display, vertex_data, fragment_data, obj_data, None).unwrap()).is_none() {
            self.render_counter += 1;
        }
        let obj = WorldObject::new(self.content.world_objects.len(), Transform::new(),RenderObject::new(Some(render_name.to_string())), PhysicsObject::RigidBody(RigidBody::new(1.0)), None);
        self.content.world_objects.push(obj);
        return self.content.world_objects.len() - 1;
    }
    

    pub fn new_object_instance(&mut self, object_name: &str,render_name: &str) -> usize{
        let obj = WorldObject::new(self.content.world_objects.len(), Transform::new(),RenderObject::new(Some(render_name.to_string())), PhysicsObject::RigidBody(RigidBody::new(1.0)), None);
        self.content.world_objects.push(obj);
        return self.content.world_objects.len() - 1;
    }
    
    pub fn add_object(&mut self, obj: WorldObject) -> usize{
        self.content.world_objects.push(obj);
        return self.content.world_objects.len() - 1;
    }

    pub fn add_texture(&mut self, object_id: &ObjectId, display: &Display<WindowSurface>, tex_data: &[u8]){
        self.textures.insert(*object_id, load_texture(display, tex_data));
    }

    pub fn update_camera(&mut self, dt: f32, input_handler: &InputHandler){
        //println!("mouse pos: {}", input_handler.pos());
        if self.content.camera.is_following() {
            println!("Following {} at {}", self.content.camera.get_following().index, self.content.world_objects[self.content.camera.get_following().index].transform.position);
            self.content.camera.set_pos(self.content.world_objects[self.content.camera.get_following().index].transform.position);
        } else {
            if input_handler.is_mouse_pressed(MouseButton::Left){
                let mut yaw = 0.0;
                let mut pitch = 0.0;
                if input_handler.pos().x.abs() > 0.2{
                    yaw = 5.0*dt*input_handler.pos().x;
                }
                if input_handler.pos().y.abs() > 0.3{
                    //println!("Right angle is {}", self.camera.get_right());
                    pitch =  5.0*dt*input_handler.pos().y;
                }
                self.content.camera.update(yaw, pitch);
            }
        }
    }

    pub fn update_physics(&mut self, dt: f32, input: &InputHandler){
        self.content.time += 1.0;
        for controller in self.controllers.iter_mut(){
            controller.update(&mut self.content, input);
        }
        
        for obj in &mut self.content.world_objects{
            obj.update_physics(dt);
        }
    }

    pub fn init_flight_scene(window: &Window, display: &Display<WindowSurface>, size: (u32,u32)) -> Scene<'a>{
        let player_renderer: Renderer<'_>  = Renderer::init(display, include_bytes!(r"../../shaders/planet/vert.glsl"), include_bytes!(r"../../shaders/planet/frag.glsl"), include_bytes!(r"../../objects/cube.obj"), None).unwrap();
        let planet_renderer  = Renderer::init(display, include_bytes!(r"../../shaders/planet/vert.glsl"), include_bytes!(r"../../shaders/planet/frag.glsl"), include_bytes!(r"../../objects/planet.obj"), None).unwrap();
        let light_source_renderer  = Renderer::init(display, include_bytes!(r"../../shaders/planet/sun_vert.glsl"), include_bytes!(r"../../shaders/planet/sun_frag.glsl"), include_bytes!(r"../../objects/planet.obj"), None).unwrap();
        
        let skybox_params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
                    .. Default::default()
                };
        
        let skybox_renderer  = Renderer::init(display, include_bytes!(r"../../shaders/skybox/vert.glsl"), include_bytes!(r"../../shaders/skybox/frag.glsl"), include_bytes!(r"../../objects/cube.obj"), Some(skybox_params)).unwrap();
        
        let mut render_map = HashMap::new();
        render_map.insert("player".to_string(), player_renderer);
        render_map.insert("light".to_string(), light_source_renderer);
        render_map.insert("skybox".to_string(), skybox_renderer);
        
        let mut player = WorldObject::new(0,  Transform::new(), RenderObject::new(Some("player".to_string())), PhysicsObject::RigidBody(RigidBody::new(1.0)), None);
        player.transform.translate(Vec3::new(1.0, 0.0, 5.0));

         let mut planet1 = WorldObject::new(1, Transform::new(), RenderObject::new(Some("light".to_string())), PhysicsObject::RigidBody(RigidBody::new(2.0)), None);
        planet1.transform.translate(Vec3::new(0.0, 0.0, 0.0));

        let mut player_movement: Movement<PlayerMover> = Movement::new(PlayerMover::new());
        player_movement.add_single(&player);
        
        let mut skybox = WorldObject::new(2, Transform::new(), RenderObject::new(Some("skybox".to_string())), PhysicsObject::StaticBody(StaticBody::new(0.0)), None);
        skybox.transform.scale(Vec3::new(40.0, 40.0, 40.0));
                

        let things = vec![player, planet1, skybox];


        let (world_texture, depth_world_texture) = create_render_textures(&display,size.0, size.1);


        let mut return_scene = Scene { content: SceneContent{ time: 0.0, world_objects: things, camera: RenderCamera::new(Vec3::new(0.0, 0.0, 15.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0), window.inner_size().into()), lights: Vec::new()}, render_counter: render_map.len() , renderers: render_map, controllers: vec![Box::new(player_movement)],  textures: HashMap::new(), scene_tex: world_texture, scene_depth: depth_world_texture};
        
        return_scene.add_texture(&ObjectId::new(0), display, include_bytes!(r"../../textures/icon.png"));
        return_scene.add_texture(&ObjectId::new(1), display, include_bytes!(r"../../textures/planet.png"));
        return_scene.add_texture(&ObjectId::new(2), display, include_bytes!(r"../../textures/skybox.png"));
        return_scene.content.camera.set_following(ObjectId { index: 0 });
        return return_scene;
    }
    
    pub fn init_gravity_scene(window: &Window, display: &Display<WindowSurface>, size: (u32,u32)) -> Scene<'a>{
        let planet_renderer  = Renderer::init(display, include_bytes!(r"../../shaders/planet/vert.glsl"), include_bytes!(r"../../shaders/planet/frag.glsl"), include_bytes!(r"../../objects/planet.obj"), None).unwrap();
        let light_source_renderer  = Renderer::init(display, include_bytes!(r"../../shaders/planet/sun_vert.glsl"), include_bytes!(r"../../shaders/planet/sun_frag.glsl"), include_bytes!(r"../../objects/planet.obj"), None).unwrap();
        
        let skybox_params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
                    .. Default::default()
                };
        
        let skybox_renderer  = Renderer::init(display, include_bytes!(r"../../shaders/skybox/vert.glsl"), include_bytes!(r"../../shaders/skybox/frag.glsl"), include_bytes!(r"../../objects/cube.obj"), Some(skybox_params)).unwrap();
        
        let mut render_map = HashMap::new();
        render_map.insert("planet".to_string(), planet_renderer);
        render_map.insert("light".to_string(), light_source_renderer);
        render_map.insert("skybox".to_string(), skybox_renderer);
        
        let mut sun = WorldObject::new(0, Transform::new(), RenderObject::new(Some("light".to_string())), PhysicsObject::RigidBody(RigidBody::new(50_000.0)), None);
        sun.transform.translate(Vec3::new(0.0, 0.0, 0.0));
        
        let mut planet1 = WorldObject::new(1, Transform::new(),RenderObject::new(Some("planet".to_string())), PhysicsObject::RigidBody(RigidBody::new(2.0)), None);
        planet1.transform.translate(Vec3::new(-5.0, -3.0, 2.0));
        planet1.physics.set_velocity(Vec3::new(0.0, 3.0, 1.0));
        
        let mut planet2 = WorldObject::new(2, Transform::new(),RenderObject::new(Some("planet".to_string())), PhysicsObject::RigidBody(RigidBody::new(1.0)), None);
        planet2.transform.translate(Vec3::new(-7.0, 3.0, 0.0));
        planet2.physics.set_velocity(Vec3::new(1.0, 4.0, 0.0));
        
        let mut planet3 = WorldObject::new(3, Transform::new(),RenderObject::new(Some("planet".to_string())), PhysicsObject::RigidBody(RigidBody::new(5.0)), None);
        planet3.transform.translate(Vec3::new(5.0, -4.0, 0.0));
        planet3.physics.set_velocity(Vec3::new(1.5, 2.0, 0.0));

        let mut skybox = WorldObject::new(4, Transform::new(), RenderObject::new(Some("skybox".to_string())), PhysicsObject::StaticBody(StaticBody::new(1.0)), None);
        skybox.transform.scale(Vec3::new(40.0, 40.0, 40.0));

        let mut gravity = Gravity::new(6.67e-5);
        gravity.add(vec![&planet1, &planet2, &planet3]);

        /*let mut spline = Spline::new_empty();
        spline.insert([Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 2.1), Vec3::new(1.0, 2.0, 0.0), Vec3::new(1.0, 0.0, -0.5)]);
        spline.insert_c2(Vec3::new(-2.0, 0.0, 0.0));*/

        let loop_spline = Spline::new_circle(Vec3::ZERO, 2.0, 0.0, 0.0, 0.0);
        let mut path_controller = Path::new(loop_spline, 0.01);

        //path_controller.add(vec![&sun]);

        let mut solar_system = vec![sun, planet1, planet2, planet3, skybox];

        let mut rng = rand::rng();
        for i in 0..0{
            let mut add_planet = WorldObject::new(5+i, Transform::new(),RenderObject::new(Some("planet".to_string())), PhysicsObject::RigidBody(RigidBody::new(rng.random_range(0.1..20.0))), None);
            add_planet.transform.translate(Vec3::new(rng.random_range(-5.5..5.5), rng.random_range(-5.5..5.5), 0.0));
            add_planet.physics.set_velocity(Vec3::new(rng.random_range(-1.5..1.5), rng.random_range(-2.0..2.0), 0.0));
            solar_system.push(add_planet);
        }

        let (world_texture, depth_world_texture) = create_render_textures(&display,size.0, size.1);


        let mut return_scene = Scene { content: SceneContent { time: 0.0, world_objects: solar_system, camera: RenderCamera::new(Vec3::new(0.0, 0.0, 15.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0), window.inner_size().into()),  lights: Vec::new()}, render_counter: render_map.len() , renderers: render_map, controllers: vec![Box::new(gravity), Box::new(path_controller)], textures: HashMap::new(), scene_tex: world_texture, scene_depth: depth_world_texture};
        
        return_scene.add_texture(&ObjectId::new(0), display, include_bytes!(r"../../textures/sun.png"));
        return_scene.add_texture(&ObjectId::new(1), display, include_bytes!(r"../../textures/planet.png"));
        return_scene.add_texture(&ObjectId::new(4), display, include_bytes!(r"../../textures/skybox.png"));
        
        return return_scene;
    }

    pub fn objects(&mut self) -> &mut Vec<WorldObject> {
        &mut self.content.world_objects
    }
}

impl SceneContent{
    pub fn objects(&mut self) -> &mut Vec<WorldObject> {
        &mut self.world_objects
    }
}