use std::io::stdout;

use glam::{Mat4, Quat, Vec3};
use glium::{glutin::{api::wgl::display, surface::WindowSurface}, index::PrimitiveType, Display, IndexBuffer, VertexBuffer};

use crate::{assetmanager::RenderManager, rendering::{self, render::{Renderer, VertexSimple}}, util};

pub struct BezierCurve{
    control_points: [Vec3; 4],
}

impl BezierCurve{
    pub fn evaluate(&self, t: f32) -> Vec3{
        let p0 = (1.0-t).powf(3.0) * self.control_points[0];
        let p1 = 3.0*(1.0-t).powf(2.0)*t * self.control_points[1];
        let p2 = 3.0 * (1.0 - t) * t.powf(2.0) * self.control_points[2];
        let p3 = (t).powf(3.0)*self.control_points[3];

        return p0+p1+p2+p3;
    }

    pub fn new(points: [Vec3; 4]) -> BezierCurve{
        BezierCurve{
            control_points: points
        }
    }
}

pub struct Spline{
    curves: Vec<BezierCurve>,
}

impl Spline{
    pub fn new_empty() -> Spline{
        Spline { curves: Vec::new() }
    }

    pub fn new( points: [Vec3; 4]) -> Spline{
        Spline {
            curves: vec![BezierCurve::new(points)]
        }
    }

    pub fn insert(&mut self, points: [Vec3; 4]) {
        self.curves.push(BezierCurve::new(points));
    }

    pub fn insert_c0(&mut self, points: [Vec3; 3]){
        let n = self.curves.len();
        let insert_points = [self.curves[n-1].control_points[3], points[0], points[1], points[2]];
        self.curves.push(BezierCurve::new(insert_points));
    }

    pub fn insert_c1(&mut self, points: [Vec3; 2]) {
        let n = self.curves.len();
        let mid_point = self.curves[n-1].control_points[3];
        let mirrored_point = 2.0*mid_point - self.curves[n-1].control_points[2];
        let insert_points = [mid_point, mirrored_point, points[0], points[1]];
        self.curves.push(BezierCurve::new(insert_points));
    }

    pub fn insert_c2(&mut self, point: Vec3) {
        let n = self.curves.len();
        let mid_point = self.curves[n-1].control_points[3];
        let mirrored_point = 2.0*mid_point - self.curves[n-1].control_points[2];
        let offset_point = self.curves[n-1].control_points[1] + 4.0*(mid_point - self.curves[n-1].control_points[2])     ;
        let insert_points = [mid_point, mirrored_point, offset_point, point];
        self.curves.push(BezierCurve::new(insert_points));
    }

    pub fn new_circle(center: Vec3, radius: f32, elongation: f32, yaw: f32, pitch: f32) -> Spline{
        let intermediary_point = (4.0/3.0)*(std::f32::consts::PI/8.0).tan()*radius;
        let yaw = Quat::from_rotation_y(yaw);

        let pitch = Quat::from_axis_angle(Vec3::X, pitch);
        let mut orientation = yaw * pitch;
        orientation = orientation.normalize();
        
        let quad1 = [orientation*(center + Vec3::new(elongation+radius,0.0,0.0)), orientation*(center + Vec3::new(elongation+radius,intermediary_point,0.0)),orientation*(center + Vec3::new(intermediary_point,radius,0.0)), orientation*(center + Vec3::new(0.0,radius,0.0))];
        let quad2 = [orientation*(center + Vec3::new(0.0,radius,0.0)), orientation*(center + Vec3::new(-intermediary_point,radius,0.0)), orientation*(center + Vec3::new(-elongation-radius,intermediary_point,0.0)), orientation*(center + Vec3::new(-elongation-radius,0.0,0.0))];
        let quad3 = [orientation*(center + Vec3::new(0.0,-radius,0.0)), orientation*(center + Vec3::new(intermediary_point,-radius,0.0)), orientation*(center + Vec3::new(elongation+radius,-intermediary_point,0.0)), orientation*(center + Vec3::new(elongation+radius,0.0,0.0))];
        let quad4 = [orientation*(center + Vec3::new(-elongation-radius,0.0,0.0)), orientation*(center + Vec3::new(-elongation-radius,-intermediary_point,0.0)),orientation*(center + Vec3::new(-intermediary_point,-radius,0.0)), orientation*(center + Vec3::new(0.0,-radius,0.0))];

        Spline{
            curves: vec![BezierCurve::new(quad1), BezierCurve::new(quad2), BezierCurve::new(quad4), BezierCurve::new(quad3)]
        }
    }

    pub fn insert_loop(&mut self, path: Spline, mirrorNormal: Vec3){
        let start = path.curves.first().unwrap();
        let end = path.curves.last().unwrap();

        for bez in path.curves{
            let mut newPoints = [Vec3::ZERO; 4];
            let mut index = 0;
            for v in bez.control_points{
                newPoints[index] = -v;
                index += 1;
            }
            let newBez = BezierCurve::new(newPoints);
            self.curves.push(newBez);
            self.curves.push(bez);
        }
    }

    pub fn to_array(&self) -> Vec<Vec3>{
        let mut out_vec: Vec<Vec3> = Vec::new();
        for bez in &self.curves{
            out_vec.append(&mut bez.control_points.to_vec());
        }
        return out_vec
    }

    pub fn to_vertex(&self) -> Vec<VertexSimple>{
        let vec = self.to_array();
        let mut out_vec = Vec::new();
        for point in vec{
            out_vec.push(VertexSimple{
                w_position: [point.x,point.y,point.z],
            });
        }
        return out_vec;
    }

    pub fn get_indicies(&self) -> Vec<u16>  {
        (0..=self.to_array().len() as u16).collect()
    }

    /*pub fn next(&self, pos:Vec3) -> Vec3 {
        // Get which beziercurve
        // Take t at that beziercurve?
        // Return result
    }*/

    pub fn evaluate(&self, t:f32) -> Vec3 {
        let n = self.curves.len();
        if n == 0 {
            return Vec3::ZERO;
        }

        let t = t.clamp(0.0, n as f32 - f32::EPSILON);
        let seg_idx = t.floor() as usize;
        let u = t.fract();

        self.curves.get(seg_idx).unwrap().evaluate(u)
    }

    pub fn len(&self) -> usize{
        return self.curves.len();
    }

    pub fn spline_renderer(&self, display: &Display<WindowSurface>, render_manager: &mut RenderManager) -> (VertexBuffer<VertexSimple>, IndexBuffer<u16>, Renderer){
        let surface_vert_shader  = util::read_shader(include_bytes!(r"../../shaders/spline/bezier_curve/vert.glsl"));
        let surface_frag_shader  = util::read_shader(include_bytes!(r"../../shaders/spline/bezier_curve/frag.glsl"));
        let surface_tess_ctrl_shader  = util::read_shader(include_bytes!(r"../../shaders/spline/bezier_curve/tess_ctrl.glsl"));
        let surface_tess_eval_shader  = util::read_shader(include_bytes!(r"../../shaders/spline/bezier_curve/tess_eval.glsl"));
        let geo_shader  = util::read_shader(include_bytes!(r"../../shaders/spline/bezier_curve/geo.glsl"));

        let line_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
          //polygon_mode: glium::draw_parameters::PolygonMode::Line,
          line_width: Some(4.0),
            .. Default::default()
        };

        let mut vbo: VertexBuffer<VertexSimple> = glium::VertexBuffer::new(display, &self.to_vertex()).unwrap();
        //println!("{:?}", &self.to_vertex());
        let mut inds = glium::IndexBuffer::new(display,PrimitiveType::Patches {vertices_per_patch: 4,},
            &self.get_indicies()).unwrap();
        //println!("{:?}", &self.get_indicies());
        let surface_renderer = rendering::render::Renderer::new(render_manager, &vec![], &vec![], Some(PrimitiveType::Patches {vertices_per_patch: 4,}), &surface_vert_shader, &surface_frag_shader, /*Some(geo_shader)*/ None, Some(&surface_tess_ctrl_shader), Some(&surface_tess_eval_shader), &display, Some(line_params), None).unwrap();

        return (vbo, inds, surface_renderer);
    }
}
