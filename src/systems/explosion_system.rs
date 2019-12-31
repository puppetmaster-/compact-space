use specs::{System, WriteStorage, Join, Entities, WriteExpect, ReadStorage};
use crate::components::{Position, Lifetime, Collided, Explosive, ParticleRequest, ExplosionSound};
use crate::ressources::Randomizer;
use crate::auxiliary::{Vec2F32, degrees_to_radians};
use rand::Rng;
use crate::systems::particle_system::ParticleBuilder;
use crate::systems::sound_system::SoundBuilder;

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, Explosive>,
		ReadStorage<'a, Position>,
		WriteExpect<'a, ParticleBuilder>,
		WriteExpect<'a, SoundBuilder>,
		ReadStorage<'a, Collided>,
		WriteExpect<'a, Randomizer>,
		WriteStorage<'a, Lifetime>,
		ReadStorage<'a, ExplosionSound>,
	);

	fn run(&mut self, data : Self::SystemData) {
		let (
			entities,
			mut explosives,
			positions,
			mut particle_builder,
			mut sound_builder,
			collided,
			mut randomizer,
			mut lifetimes,
			explosion_sounds
		) = data;
		let mut deads_entities = vec![];
		for (entity, position, explosion, _) in (&*entities, &positions, &mut explosives, &collided).join() {
			for _ in 0..randomizer.rnd.gen_range(explosion.amount_range.0, explosion.amount_range.1){
				let direction = Vec2F32::new(0.0,1.0).rotated_z(degrees_to_radians(randomizer.rnd.gen_range(0.0, 360.0)));
				let texture_id = randomizer.rnd.gen_range(explosion.texture_id.0,explosion.texture_id.1);
				let velocity = randomizer.rnd.gen_range(explosion.velocity_range.0,explosion.velocity_range.1);
				let lifetime = randomizer.rnd.gen_range(explosion.lifetime_range.0,explosion.lifetime_range.1);
				//let acceleration = randomizer.rnd.gen_range(explosion.acceleration_range.0,explosion.acceleration_range.1);
				particle_builder.request(ParticleRequest{
					render_order: 3,
					direction,
					velocity,
					acceleration: 0.0,
					pos: Position { value: position.value},
					lifetime,
					texture_id,
					rotation: explosion.rotation,
					color: explosion.color
				});
			}
			if let Some(exp) = explosion_sounds.get(entity){
				sound_builder.request(exp.id,exp.vol);
			}
			deads_entities.push(entity);

			if let Some(t) = lifetimes.get_mut(entity) { t.time = 0.0 }
		}

		for dead in deads_entities.iter(){
			explosives.remove(*dead);
		}
	}
}