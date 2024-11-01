
use nalgebra_glm::{Vec3, rotate_vec3};
use std::f32::consts::PI;

pub struct Camera {
  pub ojo: Vec3,
  pub centro: Vec3,
  pub sube: Vec3,
  pub camb: bool
}

impl Camera {
  pub fn new(ojo: Vec3, centro: Vec3, coo_arr: Vec3) -> Self {
    Camera {
      ojo,
      centro,
      sube: coo_arr,
      camb: true,
    }
  }



  pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
    let radius_vector = self.ojo - self.centro;
    let radius = radius_vector.magnitude();

    let current_yaw = radius_vector.z.atan2(radius_vector.x);

    let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
    let current_pitch = (-radius_vector.y).atan2(radius_xz);

    let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
    let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

    let ojo_nuevo = self.centro + Vec3::new(
      radius * new_yaw.cos() * new_pitch.cos(),
      -radius * new_pitch.sin(),
      radius * new_yaw.sin() * new_pitch.cos()
    );

    self.ojo = ojo_nuevo;
    self.camb = true;
  }

  pub fn zoom(&mut self, delta: f32) {
    let direction = (self.centro - self.ojo).normalize();
    self.ojo += direction * delta;
    self.camb = true;
  }

  pub fn move_center(&mut self, direction: Vec3) {
    let radius_vector = self.centro - self.ojo;
    let radius = radius_vector.magnitude();

    let angle_x = direction.x * 0.05; 
    let angle_y = direction.y * 0.05;

    let rotated = rotate_vec3(&radius_vector, angle_x, &Vec3::new(0.0, 1.0, 0.0));

    let right = rotated.cross(&self.sube).normalize();
    let final_rotated = rotate_vec3(&rotated, angle_y, &right);

    self.centro = self.ojo + final_rotated.normalize() * radius;
    self.camb = true;
  }


}