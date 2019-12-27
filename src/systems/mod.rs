use specs::{DispatcherBuilder, Dispatcher};

pub mod bullet_system;
pub mod player_system;
pub mod lifetime_system;
pub mod input_system;
pub mod moving_system;
pub mod timer_system;
pub mod rotating_system;
pub mod target_system;
pub mod camera_system;
pub mod particle_system;
pub mod collision_system;
pub mod dying_system;
pub mod explosion_system;
pub mod follower_system;
pub mod spawning_system;
pub mod emitter_system;
pub mod sound_system;
pub mod visibility_system;

pub fn create_dispatcher<'a,'b>() -> Dispatcher<'a, 'b>{
	DispatcherBuilder::new()
		.with(timer_system::Sys{},"timer",&[])
		.with(visibility_system::Sys{},"visibility",&[])
		.with(spawning_system::Sys{},"spawning",&["timer"])
		.with(player_system::Sys {}, "player", &[])
		.with(lifetime_system::Sys{}, "lifetime",&[])
		.with(emitter_system::Sys{}, "emitting",&[])
		.with(bullet_system::Sys {}, "spawn_bullets", &["player"])
		.with(particle_system::Sys {}, "spawn_particles", &["player"])
		.with(target_system::Sys {}, "targeting", &["player"])
		.with(rotating_system::Sys{},"rotating",&["player"])
		.with(collision_system::Sys{},"collision",&["player"])
		.with(follower_system::Sys{},"following",&["player"])
		.with(sound_system::Sys{}, "sound",&["player"])
		.with(moving_system::Sys {}, "moveing", &["targeting","rotating"])
		.with(explosion_system::Sys{}, "explosion",&["collision"])
		.with(dying_system::Sys {}, "dying", &["moveing"])
		.with(camera_system::Sys {}, "camera", &["moveing"])
		.build()
}