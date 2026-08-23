use glam::Vec3;
use glium::{DrawParameters, PolygonMode};

use crate::{assetmanager::{CreatorManager, RenderManager}, rendering::render::{Renderer, Vertex}, scene::objects::{ObjectId, WorldObject, physics::{self, PhysicsObject, bodies::StaticBody}, renderable::MeshRenderer, transform::Transform}, util::read_shader};


pub struct GridContext {
    pub a: f32,
    pub b: f32,
    pub normal: Vec3,
    pub center: Vec3,
    pub cell_size: f32,
    pub index: usize,
    pub width: usize,
    pub height: usize,
}

pub fn paraboloid_grid(ctx: GridContext) -> ([f32; 3], [f32; 2]) {
    let normal = ctx.normal.normalize();

    let reference = if normal.dot(Vec3::Y).abs() > 0.999 {
        Vec3::X
    } else {
        Vec3::Y
    };

    let right = normal.cross(reference).normalize();
    let forward = normal.cross(right).normalize();

    let u = ctx.a as f32 / ctx.width as f32;
    let v = ctx.b as f32 / ctx.height as f32;

    let x = (ctx.a as f32 - ctx.width as f32 / 2.0) * ctx.cell_size;
    let z = (ctx.b as f32 - ctx.height as f32 / 2.0) * ctx.cell_size;

    let curvature = 0.1;

    let height = curvature * (x * x + z * z);

    let position =
        ctx.center
        + right * x
        + forward * z
        + normal * height;

    (position.to_array(), [u, v])
}


pub fn centered_grid(ctx: GridContext) -> ([f32; 3], [f32; 2]) {

    let reference = if ctx.normal.dot(Vec3::Y).abs() > 0.999 {
        Vec3::X
    } else {
        Vec3::Y
    };

    // Construct an orthonormal basis for the plane.
    let right = ctx.normal.cross(reference).normalize();
    let up = ctx.normal.cross(right).normalize();

    let u = ctx.a as f32 / ctx.width as f32;
    let v = ctx.b as f32 / ctx.height as f32;
    let x = (ctx.a as f32 - ctx.width as f32 / 2.0) * ctx.cell_size;
    let y = (ctx.b as f32 - ctx.height as f32 / 2.0) * ctx.cell_size;
    let position = ctx.center + right * x + up * y;

    return (position.to_array(), [u, v]);
}

pub fn create_generalized_grid<F>(center: Vec3, normal: Vec3, width: usize, height: usize, cell_size: f32, create_grid: F, creator: &mut CreatorManager) -> ObjectId 
where 
    F: Fn(GridContext) -> ([f32; 3], [f32; 2])
{
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();
    let transform = Transform::new_from_pos(center);
    let id = creator.next_id();
    let physics = PhysicsObject::Nothing();
    let collider = None;

    let normalized_normal = normal.normalize();
    let mut i = 0;
    for x in 0..=width {
        for z in 0..=height {
            let context = GridContext {
                a: x as f32,
                b: z as f32,
                normal: normalized_normal,
                center,
                cell_size,
                index: i,
                width,
                height
            };

            let (position, uv) = create_grid(context);

            vertices.push(Vertex {
                position: position,
                normal: normal.to_array(),
                tex_coords: uv
            });
            i += 1;
        }
    }

    let col = height + 1; 
    for x in 0..width {
        for z in 0..height {
            let top_left = x * col + z;
            let bottom_left = x * col + (z + 1);
            let top_right = (x + 1) * col + z;
            let bottom_right = (x + 1) * col + (z + 1);

            indices.extend_from_slice(&[
                top_left as u16, top_right as u16, bottom_left as u16,
                top_right as u16, bottom_right as u16, bottom_left as u16,
            ]);
        }
    }

    
    if !creator.scene.render_manager.renderers.contains_key("grid") {
        let vert_shader = read_shader(include_bytes!(r"../../../../shaders/shapes/grid_vert.glsl"));
        let frag_shader = read_shader(include_bytes!(r"../../../../shaders/shapes/grid_frag.glsl"));
        let draw_params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise, // No culling for planes ig
                    polygon_mode: PolygonMode::Line,
                    line_width: Some(500.0),
                    .. Default::default()
            };
        let renderer = Renderer::new(&mut creator.scene.render_manager, &vertices, &indices, None, vert_shader, frag_shader, None, None, None, creator.display, Some(draw_params), None).expect("Could not create renderer");

        creator.scene.render_manager.add_renderer("grid".to_string(), renderer);
    }

    let object = WorldObject::new(id, transform, Some(Box::new(MeshRenderer::new("grid".to_string()))), physics, collider);
    creator.add_object(object);
    ObjectId::new(id)
}

pub fn create_grid(center: Vec3, normal: Vec3, width: usize, height: usize, cell_size: f32, creator: &mut CreatorManager) -> ObjectId {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();
    let transform = Transform::new_from_pos(center);
    let id = creator.next_id();
    let physics = PhysicsObject::Nothing();
    let collider = None;
    
    // Create verticies centered at a point in space
    let normalized_normal = normal.normalize();
    let reference = if normalized_normal.dot(Vec3::Y).abs() > 0.999 {
        Vec3::X
    } else {
        Vec3::Y
    };

    // Construct an orthonormal basis for the plane.
    let right = normalized_normal.cross(reference).normalize();
    let up = normalized_normal.cross(right).normalize();

    for x in 0..=width {
        for z in 0..=height {
            let u = x as f32 / width as f32;
            let v = z as f32 / height as f32;
            let x = (x as f32 - width as f32 / 2.0) * cell_size;
            let z = (z as f32 - height as f32 / 2.0) * cell_size;
            let position = center + right * x + up * z;

            vertices.push(Vertex {
                position: position.to_array(),
                normal: normal.to_array(),
                tex_coords: [u, v]
            });
        }
    }
    // Simple approach each grid point consists of two triangles.

    let col = height + 1; 
    for x in 0..width {
        for z in 0..height {
            let top_left = x * col + z;
            let bottom_left = x * col + (z + 1);
            let top_right = (x + 1) * col + z;
            let bottom_right = (x + 1) * col + (z + 1);

            indices.extend_from_slice(&[
                top_left as u16, top_right as u16, bottom_left as u16,
                top_right as u16, bottom_right as u16, bottom_left as u16,
            ]);
        }
    }

    if !creator.scene.render_manager.renderers.contains_key("grid") {
        let vert_shader = read_shader(include_bytes!(r"../../../../shaders/shapes/grid_vert.glsl"));
        let frag_shader = read_shader(include_bytes!(r"../../../../shaders/shapes/grid_frag.glsl"));
        let draw_params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise, // No culling for planes ig
                    polygon_mode: PolygonMode::Line,
                    line_width: Some(10.0),
                    .. Default::default()
            };
        let renderer = Renderer::new(&mut creator.scene.render_manager, &vertices, &indices, None, vert_shader, frag_shader, None, None, None, creator.display, Some(draw_params), None).expect("Could not create renderer");

        creator.scene.render_manager.add_renderer("grid".to_string(), renderer);
    }

    let object = WorldObject::new(id, transform, Some(Box::new(MeshRenderer::new("grid".to_string()))), physics, collider);
    creator.add_object(object);
    ObjectId::new(id)
}


