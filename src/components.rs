use serde::{Serialize, Deserialize};
use specs::prelude::*;
use vek::Vec2;
use crate::auxiliary::{TextureID, Vec2F32};

// register all components
pub fn register(world: &mut World){
	world.register::<Position>();
	world.register::<Renderable>();
	world.register::<Player>();
	world.register::<Enemy>();
	world.register::<Bullet>();
	world.register::<Particle>();
	world.register::<Lifetime>();
	world.register::<Input>();
	world.register::<Moveing>();
	world.register::<Timer>();
	world.register::<Rotation>();
	world.register::<Target>();
	world.register::<Following>();
	world.register::<Hidden>();
	world.register::<Collision>();
	world.register::<Scaleable>();
	world.register::<Camera>();
	world.register::<Dying>();
	world.register::<Explosive>();
	world.register::<Collided>();
	world.register::<Spawning>();
	world.register::<DoNotDelete>();
	world.register::<Indestructible>();
	world.register::<PlaySound>();
	world.register::<Emitter>();
}

#[derive(Component, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Position {
	pub value: Vec2F32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone, Default)]
pub struct Input {
	pub throttle: bool,
	pub brake: bool,
	pub left: bool,
	pub right: bool,
	pub shoot: bool,
	pub boost: bool,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Renderable {
	pub texture_id: TextureID,
	pub render_order : i32,
	pub color: ComponentColor,
	pub origin: Vec2F32,
}
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Moveing {
	pub direction: Vec2F32,
	pub velocity: f32,
	pub max_velocity: f32,
	pub friction: f32,
	pub acceleration: f32,
}

impl Default for Moveing{
	fn default() -> Self{
		Moveing{
			direction: Vec2::zero(),
			velocity: 0.0,
			max_velocity: 0.0,
			friction: 1.0,
			acceleration: 0.0,
		}
	}
}

#[derive(Component, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Timer {
	pub value: i32,
	pub initial_value: i32,
}

#[derive(Component, Serialize, Deserialize, Clone, Default, Copy)]
#[storage(VecStorage)]
pub struct Rotation {
	pub value: f32,
	pub interval: f32,
	pub always: bool,
	pub counterclockwise: bool,
}

#[derive(Component, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Scaleable {
	pub value: Vec2F32,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Lifetime {
	pub time : f32,
	pub tick_value: f32,
}

#[derive(Component)]
pub struct Target {
	pub target: Entity,
	pub offset: Vec2F32
}

#[derive(Component)]
pub struct Following {
	pub target: Entity,
	pub rate: f64,
	pub rotation: f32,
}

#[derive(Component, Serialize, Deserialize, Debug)]
pub struct Collision {
	pub radius: f32,
	//pub mass: f32,
	//pub restitution: f32,
}

#[derive(Component, Serialize, Deserialize)]
pub struct Emitter {
	pub amount: i32,
	pub lifetime: f32,
	pub direction: Vec2F32,
	pub render_order: i32,
	pub texture_ids: Vec<u16>,
	pub spawn_time_range: (i32, i32),
	pub velocity_rang: (f32,f32),
	pub pos_x_range: (f32,f32),
	pub pos_y_range: (f32,f32),
	pub color_range: (f32,f32),
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ComponentColor {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

impl Default for ComponentColor{
	fn default() -> Self{
		ComponentColor{
			r: 1.0,
			g: 1.0,
			b: 1.0,
			a: 1.0
		}
	}
}

#[derive(Component, Serialize, Deserialize, Clone, Copy)]
pub struct Explosive{
	pub texture_id: (TextureID,TextureID),
	pub velocity_range: (f32,f32),
	pub lifetime_range: (f32,f32),
	pub color: ComponentColor,
	pub rotation: Rotation,
	//pub acceleration_range: (f32,f32),
	//pub particleRequest: ParticleRequest,
}

#[derive(Component, Serialize, Deserialize, Clone, Copy)]
pub struct PlaySound{
	sound_id: usize,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Spawning;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Bullet;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Particle;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Enemy;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Dying;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Hidden;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct DoNotDelete;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Camera;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Collided;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Indestructible;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyType {
	Follower,
	Follower2,
	Ballrider,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ParticleRequest {
	pub render_order: i32,
	pub direction: Vec2<f32>,
	pub velocity: f32,
	pub acceleration: f32,
	pub pos: Position,
	pub lifetime: f32,
	pub texture_id: TextureID,
	pub rotation: Rotation,
	pub color: ComponentColor,
}