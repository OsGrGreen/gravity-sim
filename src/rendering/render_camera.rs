
use core::f32;

use glam::{Mat4, Quat, Vec3, Vec4, Vec4Swizzles};
use winit::window::Window;

use super::render::calculate_perspective;


pub struct RenderCamera{
    pos: Vec3,
    target: Vec3,
    up: Vec3,
    front: Vec3,
    orientation: Quat, // full camera rotation
    radius: f32,
    pub perspective: Mat4,
    pub matrix: Mat4,
}

const SQRT3:f32 = 1.7320508;

impl RenderCamera{

    pub fn new(start_pos: Vec3, target:Vec3, up:Vec3, front:Vec3, dim: (f32, f32)) -> RenderCamera{

        let mut camera = RenderCamera{pos:start_pos,target:target,up:up, front:front, perspective:Mat4::ZERO, orientation : Quat::IDENTITY, radius: 20.0, matrix: Mat4::ZERO};
        camera.matrix = camera.look_at();
        camera.perspective = calculate_perspective(dim);
        return camera;
    }

    pub fn init(dim: (f32, f32)) -> RenderCamera{
        let mut camera = RenderCamera{pos:Vec3::ZERO,target:Vec3::new(0.0, 0.0, -2.0),up:Vec3::new(0.0, 1.0, 0.0), front:Vec3::new(0.0, 0.0, -1.0), perspective:Mat4::ZERO, orientation : Quat::IDENTITY, radius: 20.0, matrix: Mat4::ZERO};
        camera.matrix = camera.look_at();
        camera.perspective = calculate_perspective(dim);
        return camera;
    }

    pub fn update(&mut self, yaw_updated: f32, pitch_updated: f32){
        //println!("pos before  {}", self.pos);
       // println!("Target is {}", self.target);
        if yaw_updated == 0.0 && pitch_updated == 0.0{
            return
        }

        // rotate around WORLD up for yaw
        let yaw = Quat::from_rotation_y(yaw_updated);

        // rotate around camera's LOCAL right for pitch
        let right = self.orientation * Vec3::X;
        let pitch = Quat::from_axis_angle(right, pitch_updated);

        // combine: apply yaw first, then pitch
        self.orientation = yaw * pitch * self.orientation;
        self.orientation = self.orientation.normalize();

        /* 
        self.yaw += yaw_updated;
        self.pitch = f32::clamp(self.pitch + pitch_updated, -89.0,89.0);
        let yaw_r = self.yaw.to_radians();
        let pitch_r = self.pitch.to_radians();

        // LH system: swap sin sign compared to RH
        let front = Vec3::new(
            yaw_r.cos() * pitch_r.cos(),
            pitch_r.sin(),
            yaw_r.sin() * pitch_r.cos(),  // same, but LH means +Z is forward
        );
        println!("Prev front was {}, now it is {}", self.front, front.normalize());
        self.front = front.normalize();

        // Right handed cross flips too → use LH up
        let right = Vec3::Y.cross(self.front).normalize(); 
        self.up = self.front.cross(right).normalize();
        */

    }

    pub fn get_position(&self) -> Vec3 {
        let offset = self.orientation * Vec3::new(0.0, 0.0, -self.radius);
        self.target + offset
    }


    pub fn getMatrix(&self) -> [[f32; 4]; 4] {
        let pos = self.get_position();
        let forward = (self.target - pos).normalize();

        // Derive right and up from orientation
        let right = (self.orientation * Vec3::X).normalize();
        let up = right.cross(forward).normalize();

        return Mat4::look_at_lh(pos, self.target, up).to_cols_array_2d();
    }

    pub fn get_right(&self) -> Vec3{
        return self.front.cross(self.up).normalize();
    }

    pub fn get_pos(&self) -> Vec3{
        return self.pos
    }

    pub fn get_up(&self) -> Vec3{
        return self.up
    }

    pub fn get_front(&self) -> Vec3{
        return self.front
    }

    pub fn set_front(&mut self, new_front:Vec3){
        self.front = new_front;
    }


    pub fn r#set_x(&mut self, x:f32){
        self.pos[0] = x;
    }

    pub fn r#set_y(&mut self, y:f32){
        self.pos[1] = y;
    }

    pub fn set_pos(&mut self, pos:Vec3){
        self.pos = pos;
        self.matrix = self.look_at();
    }

    pub fn r#move(&mut self, direction:Vec3){
        self.pos += direction;
    }

    pub fn r#move_target(&mut self, direction:Vec3){
        self.target += direction;
    }

    pub fn r#change_target(&mut self, new_target:Vec3){
        self.target = new_target;
    }

    pub fn look_at(&self) -> Mat4{
        let f = (self.target-self.pos).normalize();
        let mut u = self.up.normalize();
        let s = (f.cross(u)).normalize();
        u = s.cross(f);

        let rotation = Mat4::from_cols(
            s.extend(0.0),
            u.extend(0.0),
            f.extend(0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );    
        let translation = Mat4::from_translation(-self.pos);

        rotation * translation
    }

    pub fn look_at_glm(pos:Vec3, target:Vec3,up:Vec3) -> Mat4{
        let f = (target-pos).normalize();
        let mut u = up.normalize();
        let s = (f.cross(u)).normalize();
        u = s.cross(f);

        let rotation = Mat4::from_cols(
            s.extend(0.0),
            u.extend(0.0),
            f.extend(0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );    
        let translation = Mat4::from_translation(-pos);

        rotation * translation
    }

    pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
        let f = {
            let f = direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };
    
        let s = [up[1] * f[2] - up[2] * f[1],
                 up[2] * f[0] - up[0] * f[2],
                 up[0] * f[1] - up[1] * f[0]];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };

    
        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                 f[2] * s_norm[0] - f[0] * s_norm[2],
                 f[0] * s_norm[1] - f[1] * s_norm[0]];
    
        let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
                 -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
                 -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
    
        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }

}