
use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;

use crate::FastNoiseLite;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * transformed_position;

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal
    }
}

 

pub fn shader_agua(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let agua_1 = Color::new(0, 105, 148);  
    let agua_2 = Color::new(0, 191, 255);  
    let color_h = Color::new(173, 216, 230);  

    let position = fragment.vertex_position;
    let t = uniforms.time as f32 * 0.02;  

    let ruido = ruido_fractal(&uniforms.noise, position.x + t, position.y + t, 5, 2.0, 0.5);

    let olas = (1.0 + ruido) * 0.5; 
    let base_color = agua_1.lerp(&agua_2, olas);
    let final_color = base_color.lerp(&color_h, ruido.abs());

    final_color * fragment.intensity
}

fn ruido_fractal(noise: &FastNoiseLite, x: f32, y: f32, octaves: u32, lacunarity: f32, gain: f32) -> f32 {
    let mut total = 10.0;
    let mut frequency = 20.0;
    let mut amplitude = 30.0;
    let mut max_value = 0.0; 

    for _ in 0..octaves {
        total += noise.get_noise_2d(x * frequency, y * frequency) * amplitude;
        max_value += amplitude;

        amplitude *= gain;
        frequency *= lacunarity;
    }

    total / max_value 
}
 
pub fn volcanico_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let color_roca = Color::new(169, 169, 169);
    let color_magma = Color::new(255, 69, 0);
    let ruido = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 10.0, fragment.vertex_position.y * 10.0);
    let factor = ((ruido + 1.0) / 2.0).powf(3.0);
    color_roca.lerp(&color_magma, factor) * fragment.intensity
}

pub fn oceano_profundo_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let color_superficie = Color::new(0, 105, 148);
    let color_profundidad = Color::new(0, 34, 102);
    let profundidad = (fragment.vertex_position.y * 5.0).sin() * 0.5 + 0.5;
    color_superficie.lerp(&color_profundidad, profundidad) * fragment.intensity
}

pub fn desierto_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let arena_clara = Color::new(237, 201, 175);
    let arena_oscura = Color::new(210, 180, 140);
    let ruido = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 5.0, fragment.vertex_position.y * 5.0);
    let factor = (ruido * 0.5 + 0.5).powf(2.0);
    arena_clara.lerp(&arena_oscura, factor) * fragment.intensity
}

pub fn hielo_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let hielo = Color::new(173, 216, 230);
    let grieta = Color::new(224, 255, 255);
    let ruido = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 15.0, fragment.vertex_position.y * 15.0);
    let factor = (ruido * ruido).clamp(0.0, 1.0);
    hielo.lerp(&grieta, factor) * fragment.intensity
}

pub fn jungla_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let verde_oscuro = Color::new(34, 139, 34);
    let verde_claro = Color::new(50, 205, 50);
    let ruido = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 20.0, fragment.vertex_position.y * 20.0);
    let factor = ((ruido + 1.0) / 2.0).powf(1.5);
    verde_oscuro.lerp(&verde_claro, factor) * fragment.intensity
}

pub fn metano_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let azul_gas = Color::new(0, 153, 204);
    let azul_oscuro = Color::new(0, 51, 102);
    let ruido = uniforms.noise.get_noise_2d(fragment.vertex_position.x, fragment.vertex_position.y);
    let factor = (ruido * 0.5 + 0.5).powf(3.0);
    azul_gas.lerp(&azul_oscuro, factor) * fragment.intensity
}

pub fn rocoso_montanoso_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let roca = Color::new(139, 69, 19);
    let nieve = Color::new(255, 250, 250);
    let altura = (fragment.vertex_position.y * 3.0).sin() * 0.5 + 0.5;
    let ruido = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 10.0, fragment.vertex_position.y * 10.0);
    let factor = ((ruido + 1.0) / 2.0) * altura;
    roca.lerp(&nieve, factor) * fragment.intensity
}

pub fn aurora_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let verde = Color::new(0, 255, 127);
    let morado = Color::new(75, 0, 130);
    let amarillo = Color::new(255, 255, 0);
    let ruido = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 5.0, fragment.vertex_position.y * 5.0);
    let factor = ((ruido + 1.0) / 2.0).powf(2.0);
    verde.lerp(&morado, factor).lerp(&amarillo, factor) * fragment.intensity
}

pub fn crateres_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let gris_oscuro = Color::new(169, 169, 169);
    let gris_claro = Color::new(211, 211, 211);
    let ruido = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 15.0, fragment.vertex_position.y * 15.0);
    let factor = (ruido * ruido).powf(1.5);
    gris_oscuro.lerp(&gris_claro, factor) * fragment.intensity
}


