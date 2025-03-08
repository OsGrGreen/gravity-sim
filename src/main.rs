#[macro_use]
extern crate glium;
extern crate winit;

use rand::Rng;
use glam::{Mat4, Vec2, Vec3};
use scene::{objects::{physics::physics_object_factory, renderable::{point::WorldPoint, renderobjects::RenderObject}, WorldObject}, Scene};
use util::{input_handler::InputHandler, load_icon, ray_library::{distance_ray_point, ndc_to_direction, ndc_to_point}, read_model};
use winit::{event::{MouseButton, MouseScrollDelta}, event_loop::{ControlFlow, EventLoop}, keyboard, window::{Fullscreen, Window}};
use glium::{framebuffer::SimpleFrameBuffer, glutin::surface::WindowSurface, implement_vertex, index::PrimitiveType, texture::DepthTexture2d, uniforms::{MagnifySamplerFilter, MinifySamplerFilter}, Blend, BlendingFunction, Display, LinearBlendingFactor, Surface, Texture2d, VertexBuffer};
use core::f32;
use std::{collections::HashMap, time::Instant};

mod scene;
mod rendering;
use rendering::{render::{Renderer, Vertex, VertexSimple}, render_camera::RenderCamera, text::{format_to_exact_length, RenderedText, TextVbo}};


mod util;


#[derive(Copy, Clone, Debug)]
pub struct Attr {
    world_position: [f32; 3],
    colour: [f32; 3],
    tex_offsets: [f32;3], //x offset, y offset, scaling factor   For reading in texture atlas
}
implement_vertex!(Attr, world_position, colour, tex_offsets);

impl Attr{
    pub fn is_zero(&self) -> bool{
        if self.colour == [0.0,0.0,0.0]{
            return true
        }else{
            return false
        }
    }
}

fn init_window()-> (EventLoop<()>, Window, Display<WindowSurface>) {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop building"); 
    
    event_loop.set_control_flow(ControlFlow::Poll);
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_title("Gravity Simulator").build(&event_loop);
    
    (event_loop, window, display)
}

//Camera constants

const CAMERA_SPEED:f32 = 0.5;
const eps:f32 = 0.0001;

const CONSTANT_FACTOR:f32 = 1.0;
fn main() {


    // Input handler
    let mut input_handler = InputHandler::new();

    //Set up camera matrix
    //camera.camera_matrix = camera.look_at(camera.get_pos()+camera.get_front());
    //Create eventloop, window and display (where everything renders)
    let (event_loop, window, display) = init_window();
    //For manipulating the window
    let monitor_handle = window.primary_monitor();
    window.set_window_icon(Some(load_icon(include_bytes!(r"../textures/icon.png"))));
    window.set_fullscreen(Some(Fullscreen::Borderless(monitor_handle)));
    //The camera
    let mut camera = RenderCamera::new(Vec3::new(0.0,0.5,4.5), Vec3::new(0.0,0.0,0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0,0.0,-1.0), window.inner_size().into());



    /*Loading the shaders from the files */
    let low_res_vert = util::read_shader(include_bytes!(r"../shaders/resolution/vert.glsl"));
    let low_res_frag = util::read_shader(include_bytes!(r"../shaders/resolution/frag.glsl"));

    /*Draw paramters for the different renderers */
    /*For example if a line or not */
    /*If alpha is needed, and the depthtest */


    //Read textures
    
        //Font textures
        // Font chars are of size 12 x 6
    let font_raw_image = image::load(std::io::Cursor::new(&include_bytes!(r"../textures/standard_font.png")),
    image::ImageFormat::Png).unwrap().to_rgba8();
    let font_dimensions = font_raw_image.dimensions();
    let font_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&font_raw_image.into_raw(), font_dimensions);
    let font_atlas = glium::texture::Texture2d::new(&display, font_image).unwrap();


    /*The point that symbolises where the mouse is! */
    let mut point = WorldPoint::new(0.5,Vec2::ZERO,Vec3::ZERO);

    //Quad that covers the whole screen
    let screen_quad:Vec<Vertex> = vec![
            Vertex{position: [-1.0, -1.0, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 0.0]}, 
            Vertex{position: [1.0, -1.0, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 0.0]},
            Vertex{position: [1.0, 1.0, 0.0], normal: [0.0,0.0,0.0], tex_coords: [1.0, 1.0]},
            Vertex{position: [-1.0, 1.0, 0.0], normal: [0.0,0.0,0.0], tex_coords: [0.0, 1.0]}
    ];
    
    let quad_indicies = vec![0, 2, 1, 0, 2, 3];

    //The different "renderers"
    // Is basically a combination of VBO, IndexBuffer and Program
    // Is a handy way to have everything in one place..
    let low_res_renderer = rendering::render::Renderer::new(&screen_quad,&quad_indicies, Some(glium::index::PrimitiveType::TrianglesList), &low_res_vert, &low_res_frag, None, None, None, &display, None, None).unwrap();

    camera.perspective = rendering::render::calculate_perspective(window.inner_size().into());
    

    /*Some random variables that are used through out the program */
    let mut t: f32 = 0.0;
    let dt: f32 = 0.0167;
    let mut current_time = Instant::now();
    let mut accumulator: f32 = 0.0;
    let mut total_fps: usize = 0;
    let mut timer = Instant::now();
    let mut overall_fps = 0.0;
    let mut rng = rand::rng();
    let smoothing = 0.6;  //For fps
    let mut frames:f32 = 0.0;

    let (mut world_texture, mut depth_world_texture) = create_render_textures(&display, window.inner_size().width/2, window.inner_size().height/2);
    //let mut world_obj = WorldObject::new(Renderable::new(), PhysicsObject::new());
    
    //world_obj.render_object.set_size(0.1);
    //world_obj.render_object.translate(Vec3 { x: 0.0, y: 0.0, z: -2.0 });

    let mut return_scene = Scene::init_gravity_scene(&window, &display);
    //scene_test.new_object(render_name, &display, vertex_data, fragment_data, obj_data)
    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                /*Window exit event */
                println!("Average fps was: {}", total_fps/frames as usize);
                window_target.exit()
            },
            winit::event::WindowEvent::CursorMoved { device_id: _, position } => {
                println!("Moved mouse!");
                input_handler.update_mouse(position, &window.inner_size());
            }
            winit::event::WindowEvent::MouseWheel { device_id: _, delta, phase } =>{
                    match delta {
                        MouseScrollDelta::LineDelta(x, y) => {
                        }
                        _ => {}
                    }
            }
            winit::event::WindowEvent::MouseInput { device_id: _, state, button } =>{

            }

            // TODO
            // Make input a little bit nicer
            winit::event::WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } =>{
                if event.physical_key == keyboard::KeyCode::Escape && event.state.is_pressed(){
                    println!("Average fps was: {}", total_fps/frames as usize);
                    window_target.exit()
                } 
                input_handler.update_input(event);

                //Handle inputs ends

            },
            winit::event::WindowEvent::Resized(window_size) => {
                camera.perspective = rendering::render::calculate_perspective(window_size.into());
                display.resize(window_size.into());
                (world_texture,depth_world_texture) = create_render_textures(&display, window.inner_size().width/2, window.inner_size().height/2);
            },
            winit::event::WindowEvent::RedrawRequested => {
                println!();
                let mainTimer = Instant::now();
                //Physics step
                let new_time = Instant::now();
                let mut frame_time = current_time.elapsed().as_secs_f32() - new_time.elapsed().as_secs_f32();

                if frame_time > 0.25{
                    frame_time = 0.25;
                }
                current_time = new_time;

                accumulator += frame_time;

                while accumulator >= dt {
                    let physicsTimer = Instant::now();
                    return_scene.update_physics(dt);
                    return_scene.update_camera(dt, &input_handler);
                    println!("Physics: {:.2?}", physicsTimer.elapsed());
                    t += dt;
                    accumulator -= dt;
                }
                

                //Render step
                let delta_time = timer.elapsed().as_secs_f32();
                timer = Instant::now();
                let current = 1.0 / delta_time;
                overall_fps = ((overall_fps * smoothing) + (current * (1.0-smoothing))).min(50_000.0);
                total_fps += overall_fps as usize;       
                let mut drawTimer = Instant::now();
                let mut target = display.draw();
                let mut fbo = create_fbo(&display, &world_texture, &depth_world_texture);
                fbo.clear_color_and_depth((0.05, 0.05, 0.14, 1.0), 1.0);
                println!("After fbo creation: {:.2?}", drawTimer.elapsed());
                drawTimer = Instant::now();
                return_scene.draw(&mut fbo);
                println!("Draw scene: {:.2?}", drawTimer.elapsed());
                drawTimer = Instant::now();
                target.clear_color_and_depth((0.3, 0.6, 0.1, 1.0), 1.0);
                target.draw(&low_res_renderer.vbo, &low_res_renderer.indicies,&low_res_renderer.program, &uniform! {tex: &world_texture}, &low_res_renderer.draw_params).unwrap();
                println!("Draw to screen: {:.2?}", drawTimer.elapsed());
                target.finish().unwrap();
                println!("Finish: {:.2?}", drawTimer.elapsed());
                frames = frames + 1.0;
                println!("After main: {:.2?}", mainTimer.elapsed());
                println!();
            },
            _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    });
}


/* Functions for creating the low-res image that I firstly render everything to */
/* Is mostly for the aesthetic, but also gives us a few more frames to work with */
fn create_render_textures(display: &Display<WindowSurface>, width: u32, height: u32) -> (Texture2d, DepthTexture2d) {
    let color_texture = Texture2d::empty(display, width, height).unwrap();
    let depth_texture = DepthTexture2d::empty(display, width, height).unwrap();
    (color_texture, depth_texture)
}

fn create_fbo<'a>(
    display: &'a Display<WindowSurface>,
    color_texture: &'a Texture2d,
    depth_texture: &'a DepthTexture2d,
) -> SimpleFrameBuffer<'a> {
    SimpleFrameBuffer::with_depth_buffer(display, color_texture, depth_texture).unwrap()
}