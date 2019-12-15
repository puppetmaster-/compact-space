use specs::prelude::*;
use specs::{System, WriteStorage, Join};
use crate::components::Lifetime;

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		WriteStorage<'a, Lifetime>
	);

	fn run(&mut self, data : Self::SystemData) {
		let mut lifetimes = data;
		for lifetime in (&mut lifetimes).join() {
			lifetime.time -= lifetime.tick_value;
		}
	}
}


pub fn cull_deads(world: &mut World){
	let mut deads : Vec<Entity> = Vec::new();
	{
		// Age out particles
		let lifetimes = world.write_storage::<Lifetime>();
		let entities = world.entities();
		for (entity, lifetime) in (&entities, &lifetimes).join() {
			if lifetime.time <= 0.0 {
				deads.push(entity);
			}
		}
	}
	world.delete_entities(&deads).expect("entity will not die");
}