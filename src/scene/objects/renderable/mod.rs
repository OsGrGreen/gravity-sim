use glam::{Mat4, Vec3, Vec4};
pub mod point;
pub mod renderobjects;

#[derive(Copy, Clone,Debug)]
pub struct ModelObject{
    rotation: Mat4,
    scaling: Mat4,
    translation: Mat4,
}

impl ModelObject{
    pub fn new() -> ModelObject{
        ModelObject{
            scaling: Mat4::IDENTITY,
            rotation: Mat4::IDENTITY,
            translation: Mat4::IDENTITY,
        }
    }
    pub fn new_from_pos(initial_pos: Vec3) -> ModelObject{
        ModelObject{
            scaling: Mat4::IDENTITY,
            rotation: Mat4::IDENTITY,
            translation: Mat4::from_translation(initial_pos),
        }
    }

    pub fn rotate(&mut self, axis: Vec3, angle: f32){
        let rotation_matrix = Mat4::from_axis_angle(axis, angle);
        self.rotation *= rotation_matrix;
    }

    pub fn scale(&mut self, scale: Vec3){
        let scale_matrix = Mat4::from_scale(scale);
        self.scaling *= scale_matrix;
    }

    pub fn translate(&mut self, target: Vec3){
        let translation_matrix = Mat4::from_translation(target);
        self.translation *= translation_matrix;
    }

    pub fn set_translation(&mut self, target: Vec3){
        self.translation = Mat4::from_translation(target);
    }

    pub fn get_posistion(&self) -> Vec3{
        let translation = self.translation.col(3);
        Vec3::new(translation.x, translation.y,translation.z)
    }

    // REDO THIS
    pub fn set_posistion(&mut self, new_pos: Vec3){
        let n_pos = Vec4::from_array([new_pos.x,new_pos.y,new_pos.z,1.0]);
        let new_trans = Mat4::from_cols(self.translation.col(0), self.translation.col(1), self.translation.col(2), n_pos);
        self.translation = new_trans;
    }


    pub fn get_model(self) -> Mat4{
        return self.translation*self.rotation*self.scaling
    }
}