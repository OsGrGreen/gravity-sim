use std::{any::Any, collections::{HashMap, HashSet, VecDeque}};

use glium::{Display, Texture2d, glutin::surface::WindowSurface};

use crate::{managers::handles::{MaterialHandle, MeshHandle, TextureHandle}, rendering::{Material, Mesh, render::Renderer}, scene::{Scene, objects::{ObjectId, SceneObject, WorldObject}}};

pub mod handles;

pub struct CreatorManager<'a>{
    pub display: &'a Display<WindowSurface>,
    pub scene: &'a mut Scene,
}

impl <'a> CreatorManager <'a> {
    pub fn add_object(&mut self, build: impl FnOnce(ObjectId) -> SceneObject, updatable: bool) -> ObjectId{
        self.scene.mut_objects().add(build, updatable)
    }

    pub fn new(scene: &'a mut Scene, display: &'a Display<WindowSurface>) -> Self {
        CreatorManager { display, scene }
    }
}

struct ObjectMem {
    pub object: Option<SceneObject>,
    pub generation: u32,
}

impl ObjectMem {
    pub fn new(object: Option<SceneObject>,generation: u32) -> ObjectMem {
        ObjectMem { object, generation }
    }
}

pub struct ObjectManager {
    objects: Vec<ObjectMem>,
    world_objects: Vec<ObjectId>, // Possible change to vec
    updatable_objects: Vec<ObjectId>, // Possible change to vec
    octree: f32,
    available_space: Vec<usize>,
}

impl ObjectManager {
    pub fn new() -> ObjectManager {
        ObjectManager { objects: Vec::new(), world_objects: Vec::new(), updatable_objects: Vec::new(), octree: 0.0, available_space: Vec::new() }
    }

    pub fn add(&mut self, build: impl FnOnce(ObjectId) -> SceneObject, updatable: bool) -> ObjectId{
        let id = self.allocate_id();
        let object = build(id);
        self.add_object(object, updatable)
    }

    fn allocate_id(&mut self) -> ObjectId {
        if let Some(index) = self.available_space.pop() {
            let data = &self.objects[index];
            ObjectId::new_gen(index, data.generation)
        } else {
            ObjectId::new(self.objects.len())
        }
    }

    fn add_object(&mut self, object: SceneObject, updatable: bool,) -> ObjectId {
        let id = object.id();
        if updatable {
            self.updatable_objects.push(id);
        }
        
        match &object {
            SceneObject::World(world_object) => {

                self.world_objects.push(id);
            },
            SceneObject::Spline(spline_object) => (),
        }

        match self.objects.get_mut(id.index) {
            Some(slot) => {
                assert!(slot.object.is_none());
                slot.generation += 1;
                slot.object = Some(object);
            },
            None => {
                assert!(self.objects.len() == id.index);
                self.objects.push(ObjectMem::new(Some(object), 0))
            },
        }

        id
    }

    pub fn remove_object(&mut self, object: ObjectId) -> Option<SceneObject> {
        let slot = self.objects.get_mut(object.index)?;
        if slot.generation != object.generation {
            return None; // stale
        }
        let removed = slot.object.take()?;
        slot.generation = slot.generation.wrapping_add(1); // invalidate old id

        self.world_objects.retain(|&existing| existing != object);
        self.updatable_objects.retain(|&existing| existing != object);
        self.available_space.push(object.index);

        Some(removed)
    }


    fn has_space(&self) -> bool {
        self.available_space.len() != 0
    }

    pub fn objects(&self) -> impl Iterator<Item = &SceneObject> {
        self.objects.iter().filter_map(|slot| slot.object.as_ref())
    }

    pub fn mut_objects(&mut self) -> impl Iterator<Item = &mut SceneObject> {
        self.objects.iter_mut().filter_map(|slot| slot.object.as_mut())
    }

    pub fn updatable_objects(&self) -> impl Iterator<Item = &SceneObject> {
        self.updatable_objects.iter().filter_map(|&id| self.get(id))
    }

    pub fn world_objects(&self) -> impl Iterator<Item = &WorldObject> {
        self.world_objects.iter().filter_map(|&id| self.get(id)?.as_world())
    }

    pub fn updatable_ids(&self) -> Vec<ObjectId> {
        self.updatable_objects.clone()
    }



    pub fn get(&self, id: ObjectId) -> Option<&SceneObject> {
        let slot = self.objects.get(id.index)?;
        if slot.generation != id.generation {
            return None;
        }
        slot.object.as_ref()
    }

    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut SceneObject> {
        let slot = self.objects.get_mut(id.index)?;
        if slot.generation != id.generation {
            return None;
        }
        slot.object.as_mut()
    }

    pub fn num_objects(&self) -> usize {
        self.objects.len()
    }

    pub fn num_world_objects(&self) -> usize {
        self.world_objects.len()
    }

    pub fn num_updatable(&self) -> usize {
        self.updatable_objects.len()
    }
}

pub struct RenderManager {
    pub meshes: HashMap<MeshHandle, Mesh>,
    mesh_count: usize,
    pub materials: HashMap<MaterialHandle, Material>,
    material_count: usize,
    pub textures: HashMap<TextureHandle, Texture2d>,
    texture_count: usize,
    pub renderers: HashMap<String, Renderer>,
    render_counter: usize,
    pub last_used_texture: Option<TextureHandle>,
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
            last_used_texture: None,
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