use specs::prelude::*;
use crate::components::{Bullet, Lifetime, Position, Renderable, Moveing, ComponentColor, Scaleable, Collision, Explosive};
use rand::Rng;
use tetra::math::Vec2;
use crate::ressources::Randomizer;
use crate::auxiliary::{rounded_vec2, ASSET_SIZE, WHITE};

struct BulletRequest {
	direction: Vec2<f32>,
	velocity: f32,
	pos: Vec2<f32>,
	lifetime: f32
}

pub struct BulletBuilder {
	requests : Vec<BulletRequest>,
}

impl BulletBuilder {
	pub fn new() -> BulletBuilder {
		BulletBuilder{
			requests : Vec::new(),
		}
	}

	pub fn request(&mut self, pos: Vec2<f32>, lifetime: f32, direction: Vec2<f32>, velocity: f32) {
		self.requests.push(
			BulletRequest{
				direction,
				velocity,
				pos,
				lifetime
			}
		);
	}
}

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		Entities<'a>,
		WriteExpect<'a, Randomizer>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Renderable>,
		WriteStorage<'a, Lifetime>,
		WriteStorage<'a, Moveing>,
		WriteStorage<'a, Scaleable>,
		WriteExpect<'a, BulletBuilder>,
		WriteStorage<'a, Bullet>,
		WriteStorage<'a, Collision>,
		WriteStorage<'a, Explosive>,
	);

	fn run(&mut self, data : Self::SystemData) {
		let (entities,
			mut randomizer,
			mut positions,
			mut renderables,
			mut lieftimes,
			mut moveing,
			mut scalable,
			mut bullet_builder,
			mut bullets,
			mut collisions,
			mut explosions,
		) = data;
		for new_bullet in bullet_builder.requests.iter() {
			entities.build_entity()
				.with(Position{ value: new_bullet.pos }, &mut positions)
				.with(Renderable{
					texture_id: 5,
					render_order: 1,
					color: ComponentColor { r: 0.0, g: 1.0, b: randomizer.rnd.gen_range(0.4, 0.8), a: 1.0 },
					origin:  ASSET_SIZE / 2.0, },&mut renderables)
				.with(Lifetime{ time: 2.0 , tick_value: 1.0 / new_bullet.lifetime }, &mut lieftimes)
				.with(Moveing {
					direction: rounded_vec2(new_bullet.direction),
					velocity: new_bullet.velocity + 4.0,
					max_velocity: 20.0,
					friction: 1.0,
					acceleration: 0.0 }, &mut moveing)
				.with(Scaleable{ value: Vec2::new(2.0,2.0)}, &mut scalable)
				.with(Bullet,&mut bullets)
				.with(Explosive{ texture_id: (4, 6), color: WHITE, velocity_range: (1.0, 2.0), lifetime_range: (10.0, 20.0), rotation: Default::default() }, &mut explosions)
				.with(Collision{ radius: 2.0}, &mut collisions)
				.build();
		}
		bullet_builder.requests.clear();
	}
}

