use std::{fs, str};
use std::fs::File;
use glium::glutin::surface::WindowSurface;
use glium::{Display, Texture2d};
use winit::window::Icon;

use crate::rendering::render::Vertex;

pub mod input_handler;
pub mod ray_library;

pub fn read_shader(buf: &[u8]) -> &str{

    let s = match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    return s;
}

pub fn read_model(buf: &[u8]) -> (Vec<Vertex>, Vec<u16>){
    let mut vertex_list = Vec::new();
    let mut index_list = Vec::new();

    let mut vertecies: Vec<[f32;3]> = Vec::new();
    let mut normals: Vec<[f32;3]> = Vec::new();
    let mut tex: Vec<[f32;2]> = Vec::new();

    //Maybe make this quicker in some way, but should probably be fine for this...
    let raw_string = match String::from_utf8(buf.to_vec()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let split_string: Vec<&str> = raw_string.split('\n').collect();
    let mut index_counter: u16 = 0;
    for line in split_string{
        let split_line: Vec<&str> = line.split_whitespace().collect();
        if split_line.len() > 0{
            let line_type = split_line[0];

            let mut counter = 0;
            //Maybe have hash table instead of this but i dunno
            if line_type.eq("v"){
                let mut save_value: [f32;3] = [0.0; 3];
                for i in 1..split_line.len(){
                    let val = split_line[i];
   
                    save_value[counter] = val.parse::<f32>().unwrap();
                    counter += 1;
                }
                vertecies.push(save_value);
            }else if line_type.eq("vn"){
                let mut save_value: [f32;3] = [0.0; 3];
                for i in 1..split_line.len(){
                    let val = split_line[i];
                    
                    save_value[counter] = val.parse::<f32>().unwrap();
                    counter += 1;
                }
                normals.push(save_value);
            }else if line_type.eq("vt"){
                let mut save_value: [f32;2] = [0.0; 2];
                for i in 1..split_line.len(){
                    let val = split_line[i];

                    save_value[counter] = val.parse::<f32>().unwrap();
                    counter += 1;
                }
                tex.push(save_value);
            }else if line_type.eq("f"){
                let composition: Vec<&str> =  line.split_whitespace().skip(1).collect();
                let mut indicies: Vec<u16> = Vec::new();
                for comp in composition{
                    let single:Vec<usize> = comp.split('/').map(|_value| {_value.parse::<usize>().unwrap()}).collect();
                    if single.len() == 3{
                        vertex_list.push(Vertex { position: vertecies[single[0] - 1], normal: normals[single[2] - 1], tex_coords: tex[single[1] - 1] });
                        indicies.push(index_counter);
                        index_counter += 1;
                    }
                }
                index_list.append(&mut triangulate_inds(indicies));
            }
        }

    }
    println!("Vertex list is: {:?}", vertex_list);
    println!("Index list is: {:?}", index_list);
    return (vertex_list, index_list)
}


fn triangulate_inds(inds: Vec<u16>) ->Vec<u16>{
    let mut return_inds: Vec<u16> = Vec::new();
    if inds.len() == 3{
        return inds
    }
    for i in 1..inds.len()-1{
        return_inds.push(inds[0]);
        return_inds.push(inds[i]);
        return_inds.push(inds[i+1]);
    }

    return return_inds
}

pub fn load_icon(bytes: &[u8]) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(bytes).unwrap().into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

pub fn load_texture(display: &Display<WindowSurface>, bytes: &[u8]) -> Texture2d{
    let buffer = image::load(std::io::Cursor::new(bytes),
    image::ImageFormat::Png).unwrap().to_rgba8();
    let dimensions = buffer.dimensions();
    let raw_img = glium::texture::RawImage2d::from_raw_rgba_reversed(&buffer.into_raw(), dimensions);
    glium::texture::Texture2d::new(display, raw_img).unwrap()
}