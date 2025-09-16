use std::collections::{hash_set::Iter, HashSet};

use glam::Vec2;
use winit::{dpi::{PhysicalPosition, PhysicalSize}, event::KeyEvent, keyboard::{self, KeyCode, PhysicalKey}};

pub struct InputHandler{
    movement: Vec2,
    pressed_keys: HashSet<PhysicalKey>, //Maybe make into a map and it has to be processed before it is removed...
    mouse_pos: Vec2,
    mouse_delta: Vec2,
}

impl InputHandler{
    pub fn new() -> InputHandler{
        InputHandler{
            movement: Vec2::ZERO,
            pressed_keys: HashSet::new(),
            mouse_pos: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
        }
    }

    pub fn update_mouse(&mut self, new_pos: PhysicalPosition<f64>, size: &PhysicalSize<u32>){
        let pos = Self::convert_to_ndc(new_pos, size);
        self.mouse_delta = self.mouse_pos-pos;
        self.mouse_pos = pos;
    }

    fn convert_to_ndc(pos: PhysicalPosition<f64>, size: &PhysicalSize<u32>) -> Vec2{
        Vec2::new((pos.x as f32 / size.width as f32) * 2.0 - 1.0,
        - ((pos.y as f32 / size.height as f32) * 2.0 - 1.0))
    }

    pub fn pos(&self) -> Vec2{
        self.mouse_pos
    }

    pub fn delta(&self) -> Vec2{
        self.mouse_delta
    }

    pub fn get_movement(&self) -> Vec2{
        return self.movement;
    }

    pub fn get_pressed(&self) -> Iter<PhysicalKey>{
        return self.pressed_keys.iter();
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool{
        self.pressed_keys.contains(&PhysicalKey::Code(key))
    }

    pub fn remove_pressed(&mut self, remove_key: &PhysicalKey){
        self.pressed_keys.remove(remove_key);
    }

    pub fn update_input(&mut self, event: KeyEvent){
        let key = event.physical_key;
        if event.repeat{

        }else if event.state.is_pressed(){
            //Maybe actually divide these into four different if statements..
            //Is probably faster and more clean
            //Key is Y-movement
            if key == keyboard::KeyCode::KeyS{
                self.movement[1] -= 1.0;
            }
            else if key == keyboard::KeyCode::KeyW{
                self.movement[1] += 1.0;
            } // Key is X-movement
            else if key == keyboard::KeyCode::KeyA{
                self.movement[0] -= 1.0;
            }
            else if key == keyboard::KeyCode::KeyD{
                self.movement[0] += 1.0;
            }else //Otherwise put into pressed_keys set
            {
                self.pressed_keys.insert(key);
            }
        }else{
            //Key is Y-movement
            if key == keyboard::KeyCode::KeyS{
                self.movement[1] += 1.0;
            }
            else if key == keyboard::KeyCode::KeyW{
                self.movement[1] -= 1.0;
            } // Key is X-movement
            else if key == keyboard::KeyCode::KeyA{
                self.movement[0] += 1.0;
            }
            else if key == keyboard::KeyCode::KeyD{
                self.movement[0] -= 1.0;
            }else //Otherwise put into pressed_keys set
            {
                self.pressed_keys.remove(&key);
            }
        }
    }
}