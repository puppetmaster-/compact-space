use specs::{System, WriteStorage, Join, ReadStorage, WriteExpect, Entities};
use tetra::math::Vec2;
use crate::components::{Position, Moveing, Dying, Collided, Hidden};
use crate::auxiliary::ARENA_RADIUS;
use crate::ressources::{Gamestate, State};
use crate::systems::sound_system::SoundBuilder;

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Position>,
		WriteStorage<'a, Moveing>,
		WriteExpect<'a, Gamestate>,
		WriteStorage<'a, Collided>,
		ReadStorage<'a, Dying>,
		WriteStorage<'a, Hidden>,
		WriteExpect<'a, SoundBuilder>,
	);

	fn run(&mut self, data : Self::SystemData) {
		let (
			entities,
			positions,
			mut moveing,
			mut gamestates,
			mut collided,
			dying,
			mut hidden,
			mut sound_builder,
		) = data;
		if gamestates.state == State::Running{
			for (entity, position, moves, _) in (&*entities, &positions, &mut moveing, &dying).join() {
				// kill player
				if position.value.distance(Vec2::zero()) > ARENA_RADIUS || collided.get(entity).is_some() {
					gamestates.state = State::Dead;
					moves.velocity = 0.0;
					moves.acceleration = 0.0;
					collided.remove(entity);
					hidden.insert(entity,Hidden).expect("can add to Hidden");
					sound_builder.request(6, 0.0); //turn off sound
				}
			}
		}
	}
}