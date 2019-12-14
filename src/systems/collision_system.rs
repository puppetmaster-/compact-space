use specs::{System, WriteStorage, Join, Entities, ReadStorage, WriteExpect};
use crate::components::{Position, Collision, Collided, Enemy, Hidden, Indestructible, Target};
use crate::auxiliary::Vec2F32;
use crate::systems::sound_system::SoundBuilder;

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, Collision>,
		WriteStorage<'a, Collided>,
		ReadStorage<'a, Indestructible>,
		ReadStorage<'a, Enemy>,
		ReadStorage<'a, Target>,
		ReadStorage<'a, Hidden>,
		WriteExpect<'a, SoundBuilder>,
	);

	fn run(&mut self, data : Self::SystemData) {
		let (entities,
			positions,
			collisions,
			mut collided,
			indestructible,
			enemies,
			targets,
			hidden,
			mut sound_builder
		) = data;
		let data1 = (&*entities, &positions, &collisions, !&collided, !&hidden).join().collect::<Vec<_>>();
		let data2 = (&*entities, &positions, &collisions, !&collided, !&hidden).join().collect::<Vec<_>>();
		//let mut colliders: Vec<(Entity, Entity)> = vec![];

		for (entity1, position1, collision1,_,_) in data1.iter(){
			for (entity2, position2, collision2,_,_) in data2.iter(){
				// enemies don't collide with each other
				if entity1 != entity2 && (enemies.get(*entity1).is_some() != enemies.get(*entity2).is_some()){
					if do_it_collide(position1.value, position2.value, collision1.radius, collision2.radius){
						if indestructible.get(*entity1).is_none(){
							collided.insert(*entity1, Collided).expect("could not insert in collided");
						}
						if indestructible.get(*entity2).is_none(){
							collided.insert(*entity2, Collided).expect("could not insert in collided");
						}
						if indestructible.get(*entity1).is_some() || indestructible.get(*entity2).is_some(){
							sound_builder.request(7,0.1);
						}
						if let Some(t) = targets.get(*entity1) {collided.insert(t.target, Collided).expect("could not insert in collided");}
						if let Some(t) = targets.get(*entity2) {collided.insert(t.target, Collided).expect("could not insert in collided");}

						//colliders.push((*entity1,*entity2));
						/*
						let distance = position1.value.distance(position2.value);
						let overlap = 0.5 * (collision1.radius + collision2.radius - distance);
						let displace = Vec2F32::new(overlap * (position1.value.x - position2.value.x) / distance, overlap *  (position1.value.y - position2.value.y) / distance);
						colliders.push((*entity1, *entity2, displace));
						//colliders.push(*entity2);
						*/
					}
				}
			}
		}
		/*
		for (c1, c2) in colliders.iter_mut() {
			match collisions.get_mut(*c1){
				Some(c) => c.collided = true,
				_ => {}
			}
			match collisions.get_mut(*c2){
				Some(c) => c.collided = true,
				_ => {}
			}
		}
		*/

			/*
		for (c1, c2, vec2) in colliders.iter_mut(){
			match positions.get_mut(*c1){
				Some(p) => {
					p.value.x += vec2.x;
					p.value.y += vec2.y;
				},
				_ => {}
			}

			match positions.get_mut(*c2){
				Some(p) => {
					p.value.x -= vec2.x;
					p.value.y -= vec2.y;
				},
				_ => {}
			}

		}
		*/
	}
}

fn do_it_collide(pos1: Vec2F32, pos2: Vec2F32, radius1: f32, radius2: f32) -> bool{
	pos1.distance(pos2) < radius1 + radius2
}