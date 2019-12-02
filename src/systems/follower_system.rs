use specs::{System, WriteStorage, Entities, ReadStorage, WriteExpect, ReadExpect};
use specs::join::Join;
use crate::components::{Position, Moveing, Following};
use crate::auxiliary::Vec2F32;
use crate::ressources::{Randomizer, Gamestate};
use rand::Rng;
use crate::ressources::State::Running;

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Following>,
		ReadStorage<'a, Position>,
		WriteStorage<'a, Moveing>,
		WriteExpect<'a, Randomizer>,
		ReadExpect<'a, Gamestate>,
	);

	fn run(&mut self, data : Self::SystemData) {
		let (entities, followers, positions, mut moveing,mut randomizer,gamestate) = data;
		for (_entity, moves, pos, follower) in (&*entities, &mut moveing, &positions, &followers).join() {
			if positions.get(follower.target).is_some() && gamestate.state == Running && randomizer.rnd.gen_bool(follower.rate){
				let follower_position = *positions.get(follower.target).expect("no follower position found");
				let new_direction = Vec2F32::new(follower_position.value.x - pos.value.x,follower_position.value.y - pos.value.y).rotated_z(follower.rotation).normalized();
				moves.direction = new_direction;
			}
		}
	}
}