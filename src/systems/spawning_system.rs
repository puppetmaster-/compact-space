use specs::{System, WriteStorage, Join, ReadStorage, WriteExpect, Entities, ReadExpect};
use tetra::math::Vec2;
use crate::components::*;
use crate::ressources::{Gamestate, State, Randomizer};
use crate::auxiliary::WHITE;
use crate::arena::GameData;
use rand::Rng;

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		Entities<'a>,
		WriteExpect<'a, Randomizer>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Renderable>,
		WriteStorage<'a, Lifetime>,
		WriteStorage<'a, Moveing>,
		WriteStorage<'a, Rotation>,
		WriteStorage<'a, Enemy>,
		WriteStorage<'a, Following>,
		WriteStorage<'a, Collision>,
		WriteStorage<'a, Explosive>,
		WriteStorage<'a, Timer>,
		WriteStorage<'a, Indestructible>,
		WriteStorage<'a, Target>,
		ReadExpect<'a, Gamestate>,
		ReadStorage<'a, Spawning>,
		WriteExpect<'a, GameData>,
		WriteStorage<'a, ExplosionSound>,
	);

	fn run(&mut self, data : Self::SystemData) {
		let (entities,
			mut randomizer,
			mut positions,
			mut renderables,
			mut lieftimes,
			mut moveing,
			mut rotation,
			mut enemies,
			mut followers,
			mut collisions,
			mut explosives,
			mut timers,
			mut indestructible,
			mut targets,
			gamestates,
			spawner,
			mut game_data,
			mut explosion_sounds,
		) = data;
		if gamestates.state == State::Running{
			for (timer,_) in (&mut timers, &spawner).join() {
				if timer.value <= 0{
					let phase = game_data.get_phase();
					let phase_positions = &game_data.phase_data[phase].positions;
					let amount = game_data.phase_data[phase].amount;
					let phase_enemy_type = &game_data.phase_data[phase].enemy_type;
					timer.value = game_data.phase_data[phase].pause;
					match phase_enemy_type{
						EnemyType::Stroller => {
							for i in 0..amount{
								entities.build_entity()
									.with(Position { value: phase_positions[i]},&mut positions)
									.with(Renderable {
										texture_id: 200,
										render_order: 3,
										color: WHITE,
										origin: Vec2::new(16.0,16.0),
									}, &mut renderables)
									.with(Rotation {
										..Default::default()
									}, &mut rotation)
									.with(Moveing{
										velocity: 0.5 + game_data.speed,
										max_velocity: 0.5 + game_data.speed,
										direction: Vec2::new(0.0,1.0),
										..Default::default()
									}, &mut moveing)
									.with(Lifetime{ time: 1.0, tick_value: 0.0 }, &mut lieftimes)
									.with(Collision{ radius: 7.0 },&mut collisions)
									.with(ExplosionSound{ id: 0, vol: 0.2 }, &mut explosion_sounds)
									.with(Following{ target: game_data.player_id, rate: 0.2, rotation: 1.2 }, &mut followers)
									.with(Explosive{ texture_id: (305, 309), velocity_range: (1.0, 2.0), lifetime_range: (10.0, 20.0), color: WHITE, rotation: Default::default() }, &mut explosives)
									.with(Enemy,&mut enemies)
									.build();}
							},
						EnemyType::Follower => {
							for i in 0..amount{
								let color = ComponentColor{ r: 0.2, g: 0.8, b: 0.8, a: 1.0 };
								entities.build_entity()
									.with(Position { value: phase_positions[i]},&mut positions)
									.with(Renderable {
										texture_id: 200,
										render_order: 3,
										color,
										origin: Vec2::new(16.0,16.0),
									}, &mut renderables)
									.with(Rotation {
										..Default::default()
									}, &mut rotation)
									.with(Moveing{
										velocity: 0.8 + game_data.speed,
										max_velocity: 0.8 + game_data.speed,
										direction: Vec2::new(0.0,1.0),
										..Default::default()
									}, &mut moveing)
									.with(Lifetime{ time: 1.0, tick_value: 0.0 }, &mut lieftimes)
									.with(Collision{ radius: 7.0 },&mut collisions)
									.with(ExplosionSound{ id: 8, vol: 1.0 }, &mut explosion_sounds)
									.with(Following{ target: game_data.player_id, rate: 1.0, rotation: 0.2 }, &mut followers)
									.with(Explosive{ texture_id: (305, 309), velocity_range: (1.0, 2.0), lifetime_range: (10.0, 20.0), color, rotation: Default::default() }, &mut explosives)
									.with(Enemy,&mut enemies)
									.build();}
						},
						EnemyType::Ballrider =>{
							for i in 0..amount{
								let entity = entities.build_entity()
									.with(Position { value: phase_positions[i]},&mut positions)
									.with(Renderable {
										texture_id: 201,
										render_order: 1,
										color: WHITE,
										origin: Vec2::new(64.0,64.0),
									}, &mut renderables)
									.with(Rotation { ..Default::default() }, &mut rotation)
									.with(Moveing{
										velocity: 0.5 + game_data.speed,
										max_velocity: 0.5  + game_data.speed,
										direction: Vec2::new(0.0,1.0),
										..Default::default()
									}, &mut moveing)
									.with(Lifetime{ time: 1.0, tick_value: 0.0 }, &mut lieftimes)
									.with(Collision{ radius: 55.0 },&mut collisions)
									.with(Indestructible,&mut indestructible)
									.with(ExplosionSound{ id: 0, vol: 0.4 }, &mut explosion_sounds)
									.with(Following{ target: game_data.player_id, rate: 0.1, rotation: -0.2 }, &mut followers)
									.with(Explosive{ texture_id: (310, 316), velocity_range: (1.0, 2.0), lifetime_range: (5.0, 10.0), color: ComponentColor{
										r: 0.5,
										g: 0.5,
										b: 0.5,
										a: 1.0
									}, rotation: Default::default() }, &mut explosives)
									.with(Enemy,&mut enemies)
									.build();
								entities.build_entity()
									.with(Position { value: Vec2::zero()},&mut positions)
									.with(Renderable {
										texture_id: 202,
										render_order: 2,
										color: WHITE,
										origin: Vec2::new(64.0,64.0),
									},&mut renderables)
									.with(Rotation { ..Default::default() }, &mut rotation)
									.with(Collision{ radius: 15.0 },&mut collisions)
									.with(Lifetime{ time: 1.0, tick_value: 0.0 }, &mut lieftimes)
									.with(Moveing{ ..Default::default() }, &mut moveing)
									.with(Enemy, &mut enemies)
									.with(Explosive{ texture_id: (305, 309), velocity_range: (1.0, 2.0), lifetime_range: (10.0, 20.0), color: WHITE, rotation: Default::default() }, &mut explosives)
									.with(Target{ target: entity, offset: Vec2::new(-20.0,randomizer.rnd.gen_range(-55.0,-45.0)) }, &mut targets)
									.build();
							}
						},
					}
				}
			}
		}
	}
}