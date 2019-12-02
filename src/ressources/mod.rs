use rand::prelude::*;
use specs::World;

use crate::systems::{bullet_system, particle_system};
use crate::ressources::camera::CameraRessource;
use crate::auxiliary::SEED;
use tetra::Context;

pub mod camera;

pub fn insert(ctx: &mut Context, world: &mut World){
	world.insert(bullet_system::BulletBuilder::new());
	world.insert(particle_system::ParticleBuilder::new());
	world.insert(Randomizer{ rnd: SeedableRng::from_seed(SEED)});
	world.insert(CameraRessource::new(ctx));
	world.insert(Gamestate{ state: State::Start});
}

pub struct Randomizer{
	pub rnd: StdRng
}

pub struct Gamestate{
	pub state: State,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
	Init,
	Start,
	Running,
	Dead,
	Pause,
	Quit,
}

