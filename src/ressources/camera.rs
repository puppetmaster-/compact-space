use vek::Vec2;
use tetra::Context;

pub struct CameraRessource {
	pub(crate) window_half: Vec2<f32>,
	save_zone: Vec2<f32>,
	delta: f32,
	speed: f32,
	new_offset: Vec2<f32>,
	pub(crate) offset: Vec2<f32>,
	smooth: bool,
}

impl CameraRessource {
	pub fn new(ctx: &mut Context) -> CameraRessource {
		let window_width = tetra::window::get_width(ctx) as f32;
		let window_height= tetra::window::get_height(ctx) as f32;
		let init_position = Vec2::new(-window_width/2.0, -window_height/2.0);
		CameraRessource {
			window_half: Vec2::new(window_width / 2.0, window_height / 2.0),
			save_zone: Vec2::new(5.0, 5.0),
			delta: 0.0,
			speed: 1.0,
			new_offset: init_position,
			offset: init_position,
			smooth: false,
		}
	}

	pub fn update(&mut self){
		if self.smooth{
			self.delta += 1.0;
			if self.delta as i32 >= self.speed as i32{
				self.delta = 0.0;
				if self.offset != self.new_offset {
					let x = if self.offset.x < self.new_offset.x {
						1.0
					}else if self.offset.x > self.new_offset.x {
						-1.0
					}else{
						0.0
					};
					let y = if self.offset.y < self.new_offset.y {
						1.0
					}else if self.offset.y > self.new_offset.y {
						-1.0
					}else{
						0.0
					};
					self.offset += Vec2::new(x,y);
				}
			}
		}
	}

	pub fn center_on(&mut self, position: Vec2<f32>) -> &mut Self{
		let new_position = position - self.window_half;
		if self.smooth{
			let dif = new_position - self.new_offset;
			if dif.x > self.save_zone.x || dif.x < -self.save_zone.x || dif.y > self.save_zone.y || dif.y < -self.save_zone.y{
				self.new_offset = new_position;
			}
			self
		}else{
			self.offset = new_position;
			self
		}
	}
}