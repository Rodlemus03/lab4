use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
use shaders::{aurora_shader, crateres_shader, desierto_shader, hielo_shader, jungla_shader, metano_shader, oceano_profundo_shader, rocoso_montanoso_shader, shader_agua, volcanico_shader};
use crate::fragment::Fragment;
use crate::color::Color;
use crate::shaders::vertex_shader;
use noise::Simplex;
use fastnoise_lite::FastNoiseLite;
 
pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite
}

pub struct UniformsSimplex {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: Simplex,
}

 

fn crear_ruido_perlin() -> FastNoiseLite {
    let mut noise = FastNoiseLite::new();

    noise.set_noise_type(Some(fastnoise_lite::NoiseType::Perlin));

    noise.set_seed(Some(100)); 
    noise.set_frequency(Some(0.030)); 

    noise.set_fractal_type(Some(fastnoise_lite::FractalType::PingPong));
    noise.set_fractal_octaves(Some(9)); 
    noise.set_fractal_lacunarity(Some(1.0)); 
    noise.set_fractal_gain(Some(0.100)); 
    noise.set_fractal_ping_pong_strength(Some(9.0)); 
    noise
}

 

fn crear_ruido_cellular_bacteria() -> FastNoiseLite {
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(fastnoise_lite::NoiseType::Cellular));
    noise.set_seed(Some(1337)); 
    noise.set_frequency(Some(0.010));  
    noise.set_cellular_distance_function(Some(fastnoise_lite::CellularDistanceFunction::EuclideanSq));  
    noise.set_cellular_return_type(Some(fastnoise_lite::CellularReturnType::Distance2Mul));  
    noise.set_cellular_jitter(Some(1.0)); 
    noise.set_fractal_type(Some(fastnoise_lite::FractalType::PingPong));  
    noise.set_fractal_octaves(Some(3));  
    noise.set_fractal_lacunarity(Some(2.0));  
    noise.set_fractal_gain(Some(1.0)); 
    noise.set_fractal_ping_pong_strength(Some(7.0)); 
    noise 
}

fn crear_ruido_cellular_agujero_negro() -> FastNoiseLite {
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(fastnoise_lite::NoiseType::Perlin));
    noise.set_seed(Some(100)); 
    noise.set_frequency(Some(0.030));  
    noise.set_fractal_type(Some(fastnoise_lite::FractalType::PingPong));  
    noise.set_fractal_octaves(Some(9));  
    noise.set_fractal_lacunarity(Some(1.0));  
    noise.set_fractal_gain(Some(1.0)); 
    noise.set_fractal_weighted_strength(Some(3.0));
    noise.set_fractal_ping_pong_strength(Some(10.0));  
    noise 
}

fn crear_ruido_camo() -> FastNoiseLite {
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(fastnoise_lite::NoiseType::OpenSimplex2));
    noise.set_seed(Some(1337)); 
    noise.set_frequency(Some(0.010));  
    noise.set_fractal_type(Some(fastnoise_lite::FractalType::Ridged));  
    noise.set_fractal_octaves(Some(9));  
    noise.set_fractal_lacunarity(Some(5.0));  
    noise.set_fractal_gain(Some(1.0)); 
    noise.set_fractal_weighted_strength(Some(7.0)); 
    noise 
}

fn crear_ruido_variado() -> FastNoiseLite {
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(fastnoise_lite::NoiseType::Cellular));
    noise.set_seed(Some(100)); 
    noise.set_frequency(Some(0.030));  
    noise.set_fractal_type(Some(fastnoise_lite::FractalType::FBm));  
    noise.set_fractal_octaves(Some(9));  
    noise.set_fractal_lacunarity(Some(1.0));  
    noise.set_fractal_gain(Some(1.0)); 
    noise.set_fractal_weighted_strength(Some(3.0)); 
    noise.set_cellular_distance_function(Some(fastnoise_lite::CellularDistanceFunction::EuclideanSq));  
    noise.set_cellular_return_type(Some(fastnoise_lite::CellularReturnType::Distance2Div));  
    noise.set_cellular_jitter(Some(1.0)); 
    noise 
}

fn crear_ruido_grupos() -> FastNoiseLite {
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(fastnoise_lite::NoiseType::Cellular));
    noise.set_seed(Some(1337)); 
    noise.set_frequency(Some(0.030));  
    noise.set_cellular_distance_function(Some(fastnoise_lite::CellularDistanceFunction::Hybrid));  
    noise.set_cellular_return_type(Some(fastnoise_lite::CellularReturnType::Distance2Sub));  
    noise.set_cellular_jitter(Some(2.0)); 
    noise.set_fractal_type(Some(fastnoise_lite::FractalType::PingPong));  
    noise.set_fractal_octaves(Some(3));  
    noise.set_fractal_lacunarity(Some(2.0));  
    noise.set_fractal_gain(Some(0.5)); 
    noise.set_fractal_ping_pong_strength(Some(1.0)); 
    noise 
}

fn crear_ruido_cellular_puntas() -> FastNoiseLite {
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(fastnoise_lite::NoiseType::Cellular));
    noise.set_seed(Some(1337)); 
    noise.set_frequency(Some(0.030));  
    noise.set_cellular_distance_function(Some(fastnoise_lite::CellularDistanceFunction::Manhattan));  
    noise.set_cellular_return_type(Some(fastnoise_lite::CellularReturnType::Distance));  
    noise.set_cellular_jitter(Some(1.0)); 
    noise 
}

fn main() {
    let window_width = 1000;
    let window_height = 800;
    let framebuffer_width = 1000;
    let framebuffer_height = 800;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "LAB 4",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x009965);

    let translation = Vec3::new(0.0, 0.0, 0.0);
    let rotation = Vec3::new(0.0, 0.0, 0.0);
    let rotation_anillos = Vec3::new(PI / 4.0, 0.0, 0.0);
    let scale = 1.0f32;

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let obj_sphere = Obj::load("assets/sphere.obj").expect("No se puede abrir el shpere.obj");
    let vertex_arrays_sphere = obj_sphere.get_vertex_array();



    let mut time = 0;
    let mut shader_actual = 1;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if window.is_key_down(Key::Key1) {
            shader_actual = 1;
        }
        if window.is_key_down(Key::Key2) {
            shader_actual = 2;
        }
        if window.is_key_down(Key::Key3) {
            shader_actual = 3;
        }
        if window.is_key_down(Key::Key4) {
            shader_actual = 4;
        }
        if window.is_key_down(Key::Key5) {
            shader_actual = 5;
        }
        if window.is_key_down(Key::Key6) {
            shader_actual = 6;
        }
        if window.is_key_down(Key::Key7) {
            shader_actual = 7;
        }
        if window.is_key_down(Key::Key8) {
            shader_actual = 8;
        }
        if window.is_key_down(Key::Key9) {
            shader_actual = 9;
        }
        if window.is_key_down(Key::Key0) {
            shader_actual = 0;
        }

        time += 1;

        handle_input(&window, &mut camera);

        framebuffer.clear();

        let model_matrix = create_model_matrix(translation, scale, rotation);
        let model_matrix_anillos = create_model_matrix(translation, scale, rotation_anillos);
        let view_matrix = create_view_matrix(camera.ojo, camera.centro, camera.sube);
        let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

  

        let uniforms_perlin = Uniforms { 
            model_matrix: model_matrix.clone(), 
            view_matrix: view_matrix.clone(), 
            projection_matrix: projection_matrix.clone(), 
            viewport_matrix: viewport_matrix.clone(),
            time,
            noise: crear_ruido_perlin() 
        };

 

        let uniforms_variado = Uniforms {
            model_matrix: model_matrix_anillos, 
            view_matrix: view_matrix.clone(), 
            projection_matrix: projection_matrix.clone(), 
            viewport_matrix: viewport_matrix.clone(),
            time,
            noise: crear_ruido_variado() 
        };

  

        let uniforms_camo = Uniforms { 
            model_matrix: model_matrix.clone(), 
            view_matrix: view_matrix.clone(), 
            projection_matrix: projection_matrix.clone(), 
            viewport_matrix: viewport_matrix.clone(),
            time,
            noise: crear_ruido_camo() 
        };

        let uniforms_cellular_puntas = Uniforms { 
            model_matrix: model_matrix.clone(), 
            view_matrix: view_matrix.clone(), 
            projection_matrix: projection_matrix.clone(), 
            viewport_matrix: viewport_matrix.clone(),
            time,
            noise: crear_ruido_cellular_puntas() 
        };

        let uniforms_cellular_agujero_negro = Uniforms { 
            model_matrix: model_matrix.clone(), 
            view_matrix: view_matrix.clone(), 
            projection_matrix: projection_matrix.clone(), 
            viewport_matrix: viewport_matrix.clone(),
            time,
            noise: crear_ruido_cellular_agujero_negro() 
        };

        let uniforms_cellular_bacteria = Uniforms { 
            model_matrix: model_matrix.clone(), 
            view_matrix: view_matrix.clone(), 
            projection_matrix: projection_matrix.clone(), 
            viewport_matrix: viewport_matrix.clone(),
            time,
            noise: crear_ruido_cellular_bacteria() 
        };

        let uniforms_cellular_grupos = Uniforms { 
            model_matrix: model_matrix.clone(), 
            view_matrix: view_matrix.clone(), 
            projection_matrix: projection_matrix.clone(), 
            viewport_matrix: viewport_matrix.clone(),
            time,
            noise: crear_ruido_grupos() 
        };

    

        framebuffer.set_current_color(0xFFDDDD);

        match shader_actual {
            1 => {
            render_shader(&mut framebuffer, &uniforms_perlin, &vertex_arrays_sphere, hielo_shader);
            }
            2 => render_shader(&mut framebuffer, &uniforms_cellular_puntas, &vertex_arrays_sphere, volcanico_shader),
            3 => render_shader(&mut framebuffer, &uniforms_perlin, &vertex_arrays_sphere, oceano_profundo_shader),
            4 => render_shader(&mut framebuffer, &uniforms_cellular_grupos, &vertex_arrays_sphere, desierto_shader),
       
            5 => render_shader(&mut framebuffer, &uniforms_perlin, &vertex_arrays_sphere, shader_agua),
            6 => render_shader(&mut framebuffer, &uniforms_cellular_bacteria, &vertex_arrays_sphere, jungla_shader),
            7 => render_shader(&mut framebuffer, &uniforms_camo, &vertex_arrays_sphere, metano_shader),
            8 => render_shader(&mut framebuffer, &uniforms_cellular_agujero_negro, &vertex_arrays_sphere, rocoso_montanoso_shader),
            9 => render_shader(&mut framebuffer, &uniforms_variado, &vertex_arrays_sphere, aurora_shader),
            _ => render_shader(&mut framebuffer, &uniforms_perlin, &vertex_arrays_sphere, crateres_shader),
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn render_shader(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    fragment_shader_fn: fn(&Fragment, &Uniforms) -> Color
) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = fragment_shader_fn(&fragment, &uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

 

fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 1.0;
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;
   
    if window.is_key_down(Key::Left) {
        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
        camera.orbit(0.0, rotation_speed);
    }

    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
        movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
        movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
        movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
        movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
        camera.move_center(movement);
    }

    if window.is_key_down(Key::Up) {
        camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {
        camera.zoom(-zoom_speed);
    }
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}