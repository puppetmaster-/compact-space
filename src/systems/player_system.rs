use specs::{System, WriteStorage, ReadStorage, Join, WriteExpect};
use crate::components::{Position, Player, Input, Renderable, Timer, Moveing, Rotation, ParticleRequest, ShootSound};
use tetra::math::{Vec2, Lerp};
use crate::systems::{bullet_system::BulletBuilder,particle_system::ParticleBuilder};
use crate::auxiliary::{rounded_vec2, degrees_to_radians};
use crate::ressources::Randomizer;
use rand::Rng;
use crate::systems::sound_system::SoundBuilder;
use tetra::graphics::Color;

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		ReadStorage<'a, Position>,
		WriteStorage<'a, Player>,
		ReadStorage<'a, Input>,
		WriteExpect<'a, BulletBuilder>,
		WriteExpect<'a, ParticleBuilder>,
		WriteExpect<'a, SoundBuilder>,
		WriteStorage<'a, Renderable>,
		WriteStorage<'a, Timer>,
		WriteStorage<'a, Moveing>,
		WriteStorage<'a, Rotation>,
		WriteExpect<'a, Randomizer>,
		ReadStorage<'a, ShootSound>,
	);

	fn run(&mut self, data : Self::SystemData) {
		let (positions,
			player,
			inputs,
			mut bullet_builder,
			mut particle_builder,
			mut sound_builder,
			mut renderable,
			mut timer,
			mut moveables,
			mut rotating,
			mut randomizer,
			shoot_sounds
		) = data;

		for (_, input, render, moves, rot, timer, pos, shoot_sound) in (&player, &inputs, &mut renderable, &mut moveables, &mut rotating, &mut timer, &positions, &shoot_sounds).join() {

			let new_direction = moves.direction;
			let mut red_value = 0.0;
			let mut throttle_color_value =0.0;
			let mut add_throttle_particle = false;

			if input.boost || input.throttle || input.brake || input.left || input.right {
				if input.boost {
					moves.velocity += 2.0;
					render.texture_id = 1;
					red_value = 0.6;
				}
				if input.throttle {
					if moves.acceleration < 0.1{
						moves.acceleration = 0.1;
						throttle_color_value = 0.2;
					}
					render.texture_id = 1;
					add_throttle_particle = true;
					sound_builder.request(6,0.6);
				} else if input.brake {
					moves.velocity -= 0.1;
					moves.acceleration = 0.0;
					render.texture_id = 1;
					sound_builder.request(6,0.0);
				}
				if input.left {
					if !input.shoot{
						rot.value -= Lerp::lerp(3.0, 6.0, 1.0 - moves.velocity / moves.max_velocity);
					}else{
						rot.value -= Lerp::lerp(1.0, 3.0, moves.velocity / moves.max_velocity);
					}
					render.texture_id = 2;
				} else if input.right {
					if !input.shoot{
						rot.value += Lerp::lerp(3.0, 6.0, 1.0 - moves.velocity / moves.max_velocity);
					}else{
						rot.value += Lerp::lerp(1.0, 3.0, moves.velocity / moves.max_velocity);
					}
					render.texture_id = 3;
				}

				//let new_direction = rounded_vec2(Vec2::new(0.0, -1.0).rotated_z(degrees_to_radians(rot.value + back)));
				let new_direction = Vec2::new(0.0, -1.0).rotated_z(degrees_to_radians(rot.value)).normalized();

				moves.direction = new_direction;

			}else{
				render.texture_id = 0;
				sound_builder.request(6,0.2);
			}
			if input.shoot && timer.value <= 0{
				let shot_direction = rounded_vec2(Vec2::new(0.0, -1.0).rotated_z(degrees_to_radians(rot.value)));
				let new_pos = rounded_vec2(pos.value-Vec2::new(0.0,10.0).rotated_z(degrees_to_radians(rot.value)));
				bullet_builder.request(new_pos, 80.0, shot_direction, moves.velocity);
				sound_builder.request(shoot_sound.id,shoot_sound.vol);
				timer.value = timer.initial_value;
			}
			if add_throttle_particle || moves.velocity > 0.0{
				particle_builder.request(ParticleRequest{
					render_order: 3,
					direction: new_direction.reflected(Vec2::one()),
					velocity: 0.0,
					acceleration: 0.0,
					pos: Position { value: pos.value },
					lifetime: 20.0,
					texture_id: 4,
					rotation: Rotation{value: rot.value,..Default::default()},
					color: Color { r: red_value, g: 0.4, b: randomizer.rnd.gen_range(0.1, 0.4), a: 0.8 }
				});
				particle_builder.request(ParticleRequest{
					render_order: 3,
					direction: new_direction.reflected(Vec2::one()),
					velocity: 0.0,
					acceleration: 0.0,
					pos: Position { value: pos.value },
					lifetime: 1.0,
					texture_id: 6,
					rotation: Rotation{value: rot.value,..Default::default()},
					color: Color { r: 0.8 + throttle_color_value, g: 0.8 + throttle_color_value, b: 0.6 + throttle_color_value, a: 1.0 }
				});
			}
		}

	}
}
