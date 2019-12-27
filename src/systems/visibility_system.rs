use specs::{Join, System, ReadStorage, WriteStorage, Entities};
use crate::components::{Position, Renderable, Hidden, DoNotDelete, Player};
use crate::auxiliary::ARENA_RADIUS;

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Renderable>,
		ReadStorage<'a, Position>,
		WriteStorage<'a, Hidden>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, DoNotDelete>,
	);

	fn run(&mut self, data : Self::SystemData) {
		let (entities, render, positions, mut hidden ,player,do_not_delete) = data;
		for (entity, pos, _) in (&*entities,&positions,&render).join(){
			if !player.get(entity).is_some() && !do_not_delete.get(entity).is_some(){//not the player and arena elements
				if pos.value.x < -ARENA_RADIUS - 50.0 || pos.value.x > ARENA_RADIUS + 50.0 || pos.value.y < -ARENA_RADIUS - 50.0 || pos.value.y > ARENA_RADIUS + 50.0{
					hidden.insert(entity,Hidden).expect("can add to Hidden");
				}else{
					if hidden.get(entity).is_some(){
						hidden.remove(entity).expect("can't remove entity");
					}
				}
			}
		}
	}
}