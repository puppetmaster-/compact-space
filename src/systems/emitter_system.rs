use specs::prelude::*;
use specs::{System, WriteStorage, Join};
use crate::components::{Emitter, Timer, ParticleRequest, Position, Rotation, ComponentColor};
use crate::systems::particle_system::ParticleBuilder;
use crate::ressources::Randomizer;
use rand::Rng;
use crate::auxiliary::Vec2F32;

pub struct Sys {}

#[allow(deprecated)]
impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		WriteStorage<'a, Emitter>,
		WriteStorage<'a, Timer>,
		WriteExpect<'a, ParticleBuilder>,
		WriteExpect<'a, Randomizer>,
	);

	fn run(&mut self, data : Self::SystemData) {
		let (
			mut emitter,
			mut timers,
			mut particle_builder,
			mut randomizer,
		) = data;
		for (emits, timer) in (&mut emitter, &mut timers).join() {
			if timer.value == 0{
				for _ in 0..emits.amount{
					let color_value = randomizer.rnd.gen_range(emits.color_range.0,emits.color_range.1);
					let particle_request = ParticleRequest{
						render_order: emits.render_order,
						direction: emits.direction,
						velocity: randomizer.rnd.gen_range(emits.velocity_rang.0,emits.velocity_rang.1),
						acceleration: 0.0,
						pos: Position { value: Vec2F32::new(
							randomizer.rnd.gen_range(emits.pos_x_range.0,emits.pos_x_range.1),
							randomizer.rnd.gen_range(emits.pos_y_range.0,emits.pos_y_range.1)
						) },
						lifetime: emits.lifetime,
						texture_id: *randomizer.rnd.choose(&emits.texture_ids).expect("?"),
						rotation: Rotation{
							value: randomizer.rnd.gen_range(0.0,360.0),
							interval: randomizer.rnd.gen_range(0.01,0.1),
							always: true,
							counterclockwise: randomizer.rnd.gen_bool(0.5),
						},
						color: ComponentColor{
							r: color_value,
							g: color_value,
							b: color_value,
							a: 2.0
						},
					};
					particle_builder.request(particle_request);
				}
				timer.value = randomizer.rnd.gen_range(emits.spawn_time_range.0, emits.spawn_time_range.1);
			}
		}
	}
}
