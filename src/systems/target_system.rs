use specs::{System, WriteStorage, Entities, ReadStorage};
use specs::join::Join;
use crate::components::{Position, Target, Moveing, Rotation};
use crate::auxiliary::rounded_vec2;
use std::f32::consts::PI;

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Target>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Moveing>,
		WriteStorage<'a, Rotation>,
	);

	fn run(&mut self, data : Self::SystemData) {
		let (entities, targets, mut positions, mut moveing,mut rotations) = data;
		for (entity, target) in (&*entities, &targets).join(){
			let target_moves = moveing.get(target.target).expect("no target moving found").clone();
			let mut moves = moveing.get_mut(entity).expect("no moving found");
			let target_position = *positions.get(target.target).expect("no target position found");
			let mut pos = positions.get_mut(entity).expect("no position found");
			let target_rot = *rotations.get(target.target).expect("no target rotation found");
			let mut rot = rotations.get_mut(entity).expect("no rotation found");

			pos.value = target_position.value + rounded_vec2(target.offset.rotated_z(PI / 180.0 * rot.value as f32));
			moves.direction = target_moves.direction;
			moves.velocity = target_moves.velocity;
			rot.value = target_rot.value;
		}
	}
}