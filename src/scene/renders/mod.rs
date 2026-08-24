use glium::Surface;

use crate::{managers::RenderManager, rendering::RenderContext, scene::objects::transform::Transform};

#[derive(Debug)]
pub struct TemporaryRender{
    pub transform: Transform,
    pub render_id: String,
    pub reset: bool
}

impl TemporaryRender {
    pub fn new(transform: Transform, render_id: String, reset: bool) -> TemporaryRender{
        TemporaryRender { transform, render_id, reset }
    }

    // Improve this in some way later... Maybe also make the Renderables handled by the manager i dunno...
    pub fn draw(&self, context: &mut RenderContext, manager: &RenderManager) {
        if let Some(renderer) = manager.renderers.get(&self.render_id) {
            let found_material = manager.materials.get(&renderer.material);
            let found_mesh = manager.meshes.get(&renderer.mesh);

            //let texture = manager.textures[self.texture]
            //if let Some(texture) = &self.material.texture {
                //fbo.draw(&renderer.vbo, &renderer.indicies, &renderer.program, &uniform! {tex: texture.sampled().magnify_filter(MagnifySamplerFilter::Nearest), projection: camera.perspective.to_cols_array_2d(), view: camera.getMatrix(), model: transform.model_matrix().to_cols_array_2d()}, &renderer.draw_params).unwrap();
            //} else {
            if let Some((material, mesh)) = found_material.zip(found_mesh) {
                let fbo = &mut context.framebuffer;
                let camera = context.camera;
                fbo.draw(&mesh.vertices, &mesh.indices, &material.program, &uniform! {projection: camera.perspective.to_cols_array_2d(), view: camera.getMatrix(), model: self.transform.model_matrix().to_cols_array_2d()}, &material.draw_params).unwrap();
            }else {
                println!("Did not find mesh or material");
            }
        }
    }
}