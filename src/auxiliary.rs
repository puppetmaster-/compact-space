use std::f32::consts::PI;
use vek::Vec2;
use crate::components::ComponentColor;

pub(crate) const ARENA_RADIUS: f32 = 470.0;
//pub(crate) const WINDOWS_HALF: Vec2<f32> = Vec2::new(1200.0 / 2.0, 640.0 / 2.0);
pub(crate) const ASSET_SIZE: Vec2<f32> = Vec2::new(16.0, 16.0);
pub(crate) const WHITE:  ComponentColor = ComponentColor { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
pub(crate) const SEED:[u8;32] = [12; 32];

pub type Vec2F32 = Vec2<f32>;
pub type TextureID = u16;

pub fn rounded_vec2(mut vec2: Vec2<f32>) -> Vec2<f32>{
	let x = (vec2.x * 10.0) as i32;
	let y = (vec2.y * 10.0) as i32;
	vec2.x = x as f32 / 10.0;
	vec2.y = y as f32 / 10.0;
	vec2
}

pub fn degrees_to_radians(degrees: f32) -> f32{
	PI / 180.0 * degrees
}

pub fn to_tetra_color(color: ComponentColor) -> tetra::graphics::Color{
	use tetra::graphics::Color;
	Color::rgba(color.r,color.g,color.b,color.a)
}

pub fn to_tetra_vec2(vec: vek::Vec2<f32>) -> tetra::glm::Vec2{
	tetra::glm::Vec2::new(vec.x,vec.y)
}
