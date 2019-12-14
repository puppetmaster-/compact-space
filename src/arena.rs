use rand::prelude::*;
use specs::prelude::*;
use tetra::math::Vec2;
use crate::components::{Position, Renderable, Player, Input, Timer, Rotation, Collision, Moveing, Camera, Dying, Explosive, DoNotDelete, Spawning, EnemyType, Emitter, Lifetime, Particle};
use crate::auxiliary::{WHITE, Vec2F32, degrees_to_radians, ARENA_RADIUS, SEED};

const ARENA_IMAGE_SIZE: f32 = 1400.0;

pub fn create(world: &mut World){
	let player = add_player(world);
	add_arena(world);
	add_visual_particles(world);
	add_spawner(world);
	world.insert(GameData{
		player_id: player,
		speed: 0.0,
		phase: 0,
		phase_data: get_all_phasedata(),
		turn: 0,
		random: SeedableRng::from_seed(SEED),
	})
}

pub fn reset(world: &mut World, _randomizer: StdRng){
	let mut to_delete = Vec::new();
	{
		let entities = world.entities();
		let do_not_delete = world.read_storage::<DoNotDelete>();
		let particle = world.read_storage::<Particle>();
		for (e, _, _) in (&entities, !&do_not_delete, !&particle).join() {
			to_delete.push(e);
		}
	}
	for del in to_delete.iter() {
		world.delete_entity(*del).expect("Deletion failed");
	}
	let player = add_player(world);
	world.write_resource::<GameData>().player_id = player;
	world.write_resource::<GameData>().phase = 0;
	world.write_resource::<GameData>().random = SeedableRng::from_seed(SEED);
	world.write_resource::<GameData>().speed = 0.0;
	world.write_resource::<GameData>().turn = 0;

	add_spawner(world);
}

fn add_spawner(world: &mut World){
	world.create_entity()
		.with(Timer{ value: 400, initial_value: 200 })
		.with(Spawning)
		.build();
}

fn add_visual_particles(world: &mut World){
	let mut randomizer: StdRng = SeedableRng::from_seed(SEED);
	world.create_entity()
		.with(Timer{ value: 0, initial_value: 100 })
		.with(Emitter{
			amount: 100,
			lifetime: 10000.0,
			direction: Vec2F32::new(1.0, 0.0),
			render_order: 0,
			texture_ids: vec![100, 115],
			spawn_time_range: (10, 50),
			velocity_rang: (0.1, 0.5),
			pos_x_range: (-1000.0, 1000.0),
			pos_y_range: (-1000.0, 1000.0),
			color_range: (0.3, 0.8),
		})
		.with(DoNotDelete)
		.with(Lifetime{ time: 1.0, tick_value: 1.0 })
		.build();
	for i in 0..120 {
		let x = ARENA_RADIUS * degrees_to_radians((i*3) as f32).cos();
		let y = ARENA_RADIUS * degrees_to_radians((i*3) as f32).sin();
		world.create_entity()
			.with(Timer { value: randomizer.gen_range(0,400), initial_value: 100 })
			.with(Emitter {
				amount: 1,
				direction: Vec2F32::new(0.0 - x,0.0 - y).normalized(),
				render_order: 11,
				lifetime: 200.0,
				texture_ids: vec![122, 123],
				spawn_time_range: (200, 400),
				velocity_rang: (0.2, 0.3),
				pos_x_range: (x+0.1, x+0.2),
				pos_y_range: (y+0.1, y+0.2),
				color_range: (0.99, 1.0),
			})
			.with(DoNotDelete)
			.build();
	}
	for i in 0..88 {
		let x = 360.0 * degrees_to_radians((i*4) as f32).cos();
		let y = 360.0 * degrees_to_radians((i*4) as f32).sin();
		world.create_entity()
			.with(Timer { value: randomizer.gen_range(0,400), initial_value: 100 })
			.with(Emitter {
				amount: 1,
				direction: Vec2F32::new(0.0 - x,0.0 - y).normalized(),
				render_order: 9,
				lifetime: 300.0,
				texture_ids: vec![130, 131],
				spawn_time_range: (200, 400),
				velocity_rang: (0.1, 0.2),
				pos_x_range: (x+0.1, x+0.2),
				pos_y_range: (y+0.1, y+0.2),
				color_range: (0.80, 0.9),
			})
			.with(DoNotDelete)
			.build();
	}
}

fn add_arena(world: &mut World){
	for i in -1..2{
		for j in -1..2 {
			if !(i==0 && j==0){
				world.create_entity()
					.with(Position { value: Vec2::new(i as f32 * ARENA_IMAGE_SIZE, j as f32 * ARENA_IMAGE_SIZE) })
					.with(Renderable {
						texture_id: 502,
						render_order: 10,
						color: WHITE,
						origin: Vec2::new(ARENA_IMAGE_SIZE/2.0, ARENA_IMAGE_SIZE/2.0),
					})
					.with(DoNotDelete)
					.build();
			}
		}
	}
	world.create_entity()
		.with(Position { value: Vec2::new(0.0,0.0) })
		.with(Renderable {
			texture_id: 500,
			render_order: 10,
			color: WHITE,
			origin: Vec2::new(ARENA_IMAGE_SIZE/2.0,ARENA_IMAGE_SIZE/2.0),
		})
		.with(DoNotDelete)
		.build();
	world.create_entity()
		.with(Position { value: Vec2::new(0.0,0.0) })
		.with(Renderable {
			texture_id: 501,
			render_order: 11,
			color: WHITE,
			origin: Vec2::new(ARENA_IMAGE_SIZE/2.0,ARENA_IMAGE_SIZE/2.0),
		})
		.with(Rotation{
			value: 0.0,
			interval: 0.3,
			always: true,
			counterclockwise: false
		})
		.with(DoNotDelete)
		.build();
}

fn add_player(world: &mut World) -> Entity{
	//let position = WINDOWS_HALF / 2.0;
	world.create_entity()
		.with(Position { value: Vec2::zero() })
		.with(Renderable {
			texture_id: 0,
			render_order: 10,
			color: WHITE,
			origin: Vec2::new(8.0,8.0),
		})
		.with(Player)
		.with(Timer {value: 0, initial_value: 6 })
		.with(Input::default())
		.with(Moveing {
			direction: Vec2::new(0.0,-1.0),
			velocity: 0.0,
			max_velocity: 4.0,
			friction: 0.98,
			acceleration: 0.0
		})
		.with(Rotation {
			..Default::default()
		})
		.with(ShootSound{ id: 2, vol: 0.1})
		.with(ExplosionSound{ id: 5, vol: 0.4 })
		.with(Camera)
		.with(Dying)
		.with(Explosive{ texture_id: (300, 304), velocity_range: (1.0, 2.0), lifetime_range: (10.0, 20.0), color: WHITE, rotation: Default::default() })
		.with(Collision{ radius: 4.0})
		.build()
}

pub struct GameData{
	pub player_id: Entity,
	pub phase: usize,
	pub phase_data: Vec<PhaseData>,
	pub speed: f32,
	pub turn: usize,
	pub random: StdRng,
}

impl GameData{
	pub fn get_phase(&mut self) -> usize{
		let mut phase = self.phase;
		if self.phase < self.phase_data.len()-1{
			self.phase +=1;
			if self.turn % 2 == 0 {
				phase = self.random.gen_range(0, self.phase_data.len());
			}
		}else{
			self.phase = 0;
			self.turn +=1;
			self.speed += 0.2;
		}
		phase
	}
}

pub struct PhaseData{
	pub pause: i32,
	pub amount: usize,
	pub enemy_type: EnemyType,
	pub positions: Vec<Vec2F32>,
}

impl PhaseData{
	pub fn new(pause: i32, enemy_type: EnemyType, positions: Vec<Vec2F32>) -> PhaseData{
		let amount = positions.len();
			PhaseData{
				pause,
				amount,
				enemy_type,
				positions
		}
	}
}

fn get_all_phasedata() ->Vec<PhaseData>{
	let mut data = vec![];

	data.push(PhaseData::new(
		100,
		EnemyType::Stroller,
		vec![Vec2F32::new(-400.0,-400.0),Vec2F32::new(400.0,400.0),Vec2F32::new(-400.0,400.0),Vec2F32::new(400.0,-400.0)]));
	data.push(PhaseData::new(
		200,
		EnemyType::Follower2,
		vec![Vec2F32::new(-400.0,0.0),Vec2F32::new(-400.0,200.0),Vec2F32::new(400.0,0.0),Vec2F32::new(400.0,200.0)]));
	data.push(PhaseData::new(
		400,
		EnemyType::Stroller,
		vec![Vec2F32::new(-400.0,-400.0),Vec2F32::new(400.0,400.0),Vec2F32::new(-400.0,400.0),Vec2F32::new(400.0,-400.0)]));
	data.push(PhaseData::new(
		1000,
		EnemyType::Ballrider,
		vec![Vec2F32::new(-400.0,-400.0),Vec2F32::new(400.0,400.0)]));
	data.push(PhaseData::new(
		100,
		EnemyType::Stroller,
		vec![
			Vec2F32::new(-400.0,-400.0),
			Vec2F32::new(400.0,400.0),
			Vec2F32::new(-400.0,400.0),
			Vec2F32::new(400.0,-400.0),
			Vec2F32::new(0.0,-400.0),
			Vec2F32::new(-400.0,0.0)
		]));
	data.push(PhaseData::new(
		200,
		EnemyType::Follower,
		vec![
			Vec2F32::new(-200.0,-400.0),
			Vec2F32::new(-100.0,-400.0),
			Vec2F32::new(0.0,-400.0),
			Vec2F32::new(100.0,-400.0),
			Vec2F32::new(200.0,-400.0),
		]));
	data.push(PhaseData::new(
		500,
		EnemyType::Ballrider,
		vec![Vec2F32::new(-500.0,0.0),Vec2F32::new(500.0,0.0)]));
	data.push(PhaseData::new(
		200,
		EnemyType::Follower,
		vec![
			Vec2F32::new(-200.0,400.0),
			Vec2F32::new(-100.0,400.0),
			Vec2F32::new(0.0,400.0),
			Vec2F32::new(100.0,400.0),
			Vec2F32::new(200.0,400.0),
		]));
	data
}
