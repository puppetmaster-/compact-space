use specs::{System, WriteStorage, Join, ReadExpect};
use crate::components::{Position, Moveing};
use tetra::math::ops::*;
use crate::ressources::{Gamestate, State};

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		WriteStorage<'a, Position>,
		WriteStorage<'a, Moveing>,
		ReadExpect<'a, Gamestate>,
	);

	fn run(&mut self, data : Self::SystemData) {
		let (mut positions, mut moveings, gamestates) = data;
		if gamestates.state == State::Dead || gamestates.state == State::Running || gamestates.state == State::Start{
			for (pos, moves) in (&mut positions, &mut moveings).join() {
				moves.acceleration = moves.acceleration.clamped(0.0,2.0);

				moves.velocity += moves.acceleration;

				pos.value.x += moves.direction.x * moves.velocity;
				pos.value.y += moves.direction.y * moves.velocity;

				moves.velocity = moves.velocity.clamped(0.0,moves.max_velocity);

				moves.acceleration -= 0.04;
				moves.velocity *= moves.friction;
			}
		}
	}
}