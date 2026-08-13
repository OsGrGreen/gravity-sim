use crate::{assetmanager::RenderManager, scene::{Scene, SceneContent, objects::{self, SplineObject, physics::{bodies::RigidBody, controllers::{line_controller::LineController, mouse_movement_controller::MouseDragController}}, renderable::{MeshRenderer, SplineRenderer}, transform::Transform}}, util::ray_library::{mouse_ray, ndc_to_intersection, ray_plane_intersection}};
use core::f32;
use std::{collections::HashMap, hash::Hash, println};

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use glium::{framebuffer::SimpleFrameBuffer, glutin::surface::WindowSurface, texture::DepthTexture2d, Display, Surface, Texture2d};
use crate::scene::objects::{physics::{PhysicsObject}, renderable::RenderObject, WorldObject};
use rand::Rng;
use winit::{event::MouseButton, keyboard, window::Window};

use crate::{rendering::{render::{self, Renderer}, render_camera::RenderCamera}, scene::objects::{ObjectId, physics::controllers::{self, Controller, gravity_controller::Gravity, movement_controller::{Movement, PlayerMover}, path_controller::{self, Path}}}, spline::Spline, util::{create_fbo, create_render_textures, input_handler::{self, InputHandler}, load_texture}};
use crate::scene::SceneObject;
use crate::scene::controllers::line_controller::Line;

impl Scene{
    pub fn init_slingshot_scene(window: &Window, display: &Display<WindowSurface>, size: (u32,u32)) -> Scene{
        let mut render_manager = RenderManager::new();
        let point_params = glium::DrawParameters {
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
            .. Default::default()
        };

        let player_renderer  = Renderer::init(&mut render_manager, display, include_bytes!(r"../../shaders/point_vert.glsl"), include_bytes!(r"../../shaders/point_frag.glsl"), include_bytes!(r"../../objects/point.obj"), Some(point_params.clone())).unwrap();
        let planet_renderer  = Renderer::init(&mut render_manager, display, include_bytes!(r"../../shaders/point_vert.glsl"), include_bytes!(r"../../shaders/point_frag.glsl"), include_bytes!(r"../../objects/point.obj"), Some(point_params)).unwrap();
        //let light_source_renderer  = Renderer::init(display, include_bytes!(r"../../shaders/planet/sun_vert.glsl"), include_bytes!(r"../../shaders/planet/sun_frag.glsl"), include_bytes!(r"../../objects/planet.obj"), None).unwrap();
            


            
        render_manager.add_renderer("player".to_string(), player_renderer);
        render_manager.add_renderer("planet".to_string(), planet_renderer);


        let mut player = WorldObject::new(0, Transform::new(), Some(Box::new(MeshRenderer::new("player".to_string()))), PhysicsObject::RigidBody(RigidBody::new(2.0)), None);
        player.data.transform.translate(Vec3::new(0.0, 0.0, 0.0));
        player.data.transform.scale(Vec3::new(0.4, 0.4, 0.4));


        let mut planet1 = WorldObject::new(1,Transform::new(), Some(Box::new(MeshRenderer::new("planet".to_string()))), PhysicsObject::RigidBody(RigidBody::new(2.0)), None);
        planet1.data.transform.translate(Vec3::new(3.0, 2.0, 0.0));

        let mut worldmouse = WorldObject::new(2, Transform::new(), Some(Box::new(MeshRenderer::new("player".to_string()))), PhysicsObject::RigidBody(RigidBody::new(2.0)), None);


        let camera = RenderCamera::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0), window.inner_size().into());
        
        let base_spline = Spline::new([Vec3::ZERO, Vec3::ZERO, Vec3::ZERO, Vec3::ZERO]);
        let render = Box::new(SplineRenderer::new("spline".to_string(), &base_spline, &mut render_manager, display));
        let spline = SplineObject::new(3, base_spline, Transform::new(), Some(render));

        let mouse = MouseDragController::new(Some(vec![player.data.id]));
        let line = LineController::new(Some(vec![Line{line: spline.data.id, line_points: vec![player.data.id, worldmouse.data.id]}]));

        //let mut player_movement: Movement<PlayerMover> = Movement::new(PlayerMover::new());
        //player_movement.add_single(&player);
        let things: Vec<objects::SceneObject> = vec![SceneObject::World(player), SceneObject::World(planet1), SceneObject::World(worldmouse), SceneObject::Spline(spline)];

        let (world_texture, depth_world_texture) = create_render_textures(&display,size.0, size.1);



        let mut return_scene= Scene { content: SceneContent { time: 0.0, world_objects: things, camera, lights: Vec::new()}, render_manager, controllers: vec![Box::new(mouse), Box::new(line)], textures: HashMap::new(), scene_tex: world_texture, scene_depth: depth_world_texture};
            
        //return_scene.add_texture(&ObjectId::new(1), display, include_bytes!(r"../../textures/planet.png"));
        return return_scene;
    }
}