use crate::{assetmanager::RenderManager, scene::{Scene, SceneContent, objects::{self, SceneObjectBehaviour, SplineObject, physics::{bodies::RigidBody, controllers::{gravity_controller::PlayerGravity, line_controller::LineController, mouse_movement_controller::MouseDragController, movement_controller::SimpleMover, prediction_controller::PredictionController}}, renderable::{MeshRenderer, SplineRenderer}, transform::Transform}}, util::ray_library::{mouse_ray, ndc_to_intersection, ray_plane_intersection}};
use core::f32;
use std::{collections::HashMap, hash::Hash, println};

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use glium::{framebuffer::SimpleFrameBuffer, glutin::surface::WindowSurface, texture::DepthTexture2d, Display, Surface, Texture2d};
use crate::scene::objects::{physics::{PhysicsObject}, renderable::RenderObject, WorldObject};
use rand::Rng;
use winit::{event::MouseButton, keyboard, window::Window};

use crate::{rendering::{render::{self, Renderer}, render_camera::RenderCamera}, scene::objects::{ObjectId, physics::controllers::{self, Controller, gravity_controller::Gravity, movement_controller::{Movement, QuatMover}, path_controller::{self, Path}}}, spline::Spline, util::{create_fbo, create_render_textures, input_handler::{self, InputHandler}, load_texture}};
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
        let planet_renderer  = Renderer::init(&mut render_manager, display, include_bytes!(r"../../shaders/planet/vert.glsl"), include_bytes!(r"../../shaders/planet/frag.glsl"), include_bytes!(r"../../objects/planet.obj"), None).unwrap();
        //let light_source_renderer  = Renderer::init(display, include_bytes!(r"../../shaders/planet/sun_vert.glsl"), include_bytes!(r"../../shaders/planet/sun_frag.glsl"), include_bytes!(r"../../objects/planet.obj"), None).unwrap();
            


            
        render_manager.add_renderer("player".to_string(), player_renderer);
        render_manager.add_renderer("planet".to_string(), planet_renderer);


        let mut player = WorldObject::new(0, Transform::new(), Some(Box::new(MeshRenderer::new("player".to_string()))), PhysicsObject::RigidBody(RigidBody::new(1.0)), None);
        player.data.transform.translate(Vec3::new(0.0, 0.0, 0.0));
        player.data.transform.scale(Vec3::new(0.4, 0.4, 0.4));
        player.physics.disable();
        

        let mut planet1 = WorldObject::new(1,Transform::new(), Some(Box::new(MeshRenderer::new("planet".to_string()))), PhysicsObject::RigidBody(RigidBody::new(12.0)), None);
        planet1.data.transform.translate(Vec3::new(3.0, 2.0, 0.0));

        let mut planet2 = WorldObject::new(4,Transform::new(), Some(Box::new(MeshRenderer::new("planet".to_string()))), PhysicsObject::RigidBody(RigidBody::new(12.0)), None);
        planet2.data.transform.translate(Vec3::new(-3.0, -2.0, 0.0));

        let worldmouse = WorldObject::new(2, Transform::new(), Some(Box::new(MeshRenderer::new("player".to_string()))), PhysicsObject::RigidBody(RigidBody::new(0.0)), None);


        let mut camera = RenderCamera::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0), window.inner_size().into());

        let camera_posistion = WorldObject::new(5, Transform::new_from_pos(Vec3::new(0.0, 0.0, 10.0)), None, PhysicsObject::Nothing(), None);
        camera.set_following(camera_posistion.data.id);

        let base_spline = Spline::new([Vec3::ZERO, Vec3::ZERO, Vec3::ZERO, Vec3::ZERO]);
        let render = Box::new(SplineRenderer::new("spline".to_string(), &base_spline, &mut render_manager, display));
        let mut spline = SplineObject::new(3, base_spline, Transform::new(), Some(render));
        spline.data_mut().transform.scale(Vec3 { x: 1.0, y: 1.0, z: 1.0 });
        spline.data.transform.translate(Vec3 { x: 0.0, y: 0.0, z: 0.0 });

        let mouse = MouseDragController::new(Some(vec![worldmouse.data.id]));
        let line = LineController::new(Some(vec![Line{line: spline.data.id, line_points: vec![player.data.id, worldmouse.data.id]}]));
        let mut gravity = PlayerGravity::new(1.0);
        gravity.add_single(&player);

        let mut movement = Movement::new(SimpleMover::new(0.25));
        movement.add_single(&camera_posistion);

        let mut prediction = PredictionController::new(250, 0.2, Box::new(gravity.clone()));
        prediction.add_single(&player);
        //let mut player_movement: Movement<PlayerMover> = Movement::new(PlayerMover::new());
        //player_movement.add_single(&player);
        let things: Vec<objects::SceneObject> = vec![SceneObject::World(player), SceneObject::World(planet1), SceneObject::World(worldmouse), SceneObject::Spline(spline), SceneObject::World(planet2), SceneObject::World(camera_posistion)];

        let (world_texture, depth_world_texture) = create_render_textures(&display,size.0, size.1);



        let mut return_scene= Scene { content: SceneContent { time: 0.0, world_objects: things, camera, lights: Vec::new(), render_objects: Vec::new()}, render_manager, controllers: vec![Box::new(mouse), Box::new(line), Box::new(gravity), Box::new(prediction), Box::new(movement)], scene_tex: world_texture, scene_depth: depth_world_texture};
            
        //return_scene.add_texture(&ObjectId::new(1), display, include_bytes!(r"../../textures/planet.png"));
        return return_scene;
    }
}