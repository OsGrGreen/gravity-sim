use std::collections::HashMap;

use glium::Texture2d;

use crate::{assetmanager::handles::{MaterialHandle, MeshHandle, TextureHandle}, rendering::{Material, Mesh, render::Renderer}};

pub mod handles;

pub struct RenderManager {
    pub meshes: HashMap<MeshHandle, Mesh>,
    mesh_count: usize,
    pub materials: HashMap<MaterialHandle, Material>,
    material_count: usize,
    pub textures: HashMap<TextureHandle, Texture2d>,
    texture_count: usize,
    pub renderers: HashMap<String, Renderer>,
    render_counter: usize,

}

impl RenderManager {
    pub fn new() -> RenderManager{
        RenderManager { 
            meshes: HashMap::default(), 
            mesh_count: 0, 
            
            materials: HashMap::default(), 
            material_count: 0, 

            textures: HashMap::default(), 
            texture_count: 0, 
            
            renderers: HashMap::default(), 
            render_counter: 0,
        }
    }
    
    
    pub fn new_mesh(&mut self, mesh: Mesh) -> MeshHandle {
        let count = self.mesh_count;
        self.mesh_count += 1;
        let handle = MeshHandle(count);
        self.meshes.insert(handle, mesh);
        MeshHandle(count)
    }
    pub fn new_material(&mut self, material: Material) -> MaterialHandle {
        let count = self.material_count;
        self.material_count += 1;
        let handle = MaterialHandle(count);
        self.materials.insert(handle, material);
        MaterialHandle(count)
    }
    pub fn new_texture(&mut self, texture: Texture2d) -> TextureHandle {
        let count = self.texture_count;
        self.texture_count += 1;
        let handle = TextureHandle(count);
        self.textures.insert(handle, texture);
        TextureHandle(count)
    }
    pub fn add_renderer(&mut self, id: String, renderer: Renderer) {
        if self.renderers.insert(id, renderer).is_none() {
            self.render_counter += 1;
        }
    }
}