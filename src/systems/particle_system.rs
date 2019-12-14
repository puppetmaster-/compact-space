use specs::prelude::*;
use crate::components::{Lifetime, Position, Renderable, Moveing, Scaleable, Particle, Rotation, ParticleRequest};
use tetra::math::Vec2;
use crate::auxiliary::*;

pub struct ParticleBuilder {
	requests : Vec<ParticleRequest>,
}

impl ParticleBuilder {
	pub fn new() -> ParticleBuilder {
		ParticleBuilder{
			requests : Vec::new(),
		}
	}

	pub fn request(&mut self, particle_request: ParticleRequest) {
		self.requests.push(particle_request);
	}
}

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Renderable>,
		WriteStorage<'a, Lifetime>,
		WriteStorage<'a, Moveing>,
		WriteStorage<'a, Rotation>,
		WriteStorage<'a, Scaleable>,
		WriteExpect<'a, ParticleBuilder>,
		WriteStorage<'a, Particle>,
	);

	fn run(&mut self, data : Self::SystemData) {
		let (entities,
			mut positions,
			mut renderables,
			mut lifetimes,
			mut moveing,
			mut rotations,
			mut scalable,
			mut particle_builder,
			mut particles) = data;
		for (lifetime, render) in (&lifetimes, &mut renderables).join(){
			render.color.a = lifetime.time;
		}

		for new_particle in particle_builder.requests.iter() {
			entities.build_entity()
				.with(new_particle.pos, &mut positions)
				.with(Renderable{
					texture_id: new_particle.texture_id,
					render_order: new_particle.render_order,
					color: new_particle.color,
					origin:  Vec2::new(8.0, 8.0) },&mut renderables)
				.with(Lifetime{ time: 2.0 , tick_value: 1.0 / new_particle.lifetime }, &mut lifetimes)
				.with(Moveing {
					direction: rounded_vec2(new_particle.direction),
					velocity: new_particle.velocity,
					max_velocity: 8.0,
					friction: 1.0,
					acceleration: new_particle.acceleration }, &mut moveing)
				.with(Scaleable{ value: Vec2::new(1.0,1.0)}, &mut scalable)
				.with(new_particle.rotation, &mut rotations)
				.with(Particle,&mut particles)
				.build();
		}
		particle_builder.requests.clear();
	}
}

