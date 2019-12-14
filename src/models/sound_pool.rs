// https://gist.github.com/17cupsofcoffee/f5082a13626ddf0030075d542262c728

use tetra::audio::{Sound, SoundInstance};
use tetra::Context;

#[derive(Clone)]
pub struct SoundPool {
	// This isn't used, but you could hold onto it in case you want
	// to resize your pool later?
	sound: Sound,
	instances: Vec<SoundInstance>,
	next: usize,
	single: bool,
}

impl SoundPool {
	pub fn new(ctx: &Context, sound: Sound, instance_count: usize) -> tetra::Result<SoundPool> {
		let mut instances = Vec::with_capacity(instance_count);

		for _ in 0..instance_count {
			instances.push(sound.spawn(ctx)?);
		}

		Ok(SoundPool {
			sound,
			instances,
			next: 0,
			single: false
		})
	}

	pub fn single(ctx: &Context, sound: Sound) -> tetra::Result<SoundPool>{

		let mut instances = Vec::with_capacity(1);
		let instance = sound.spawn(ctx)?;
		instance.set_repeating(true);
		instance.set_volume(0.0);
		instance.play();
		instances.push(instance);

		Ok(SoundPool {
			sound,
			instances,
			next: 0,
			single: true
		})
	}

	pub fn play(&mut self, volume: f32, speed: f32) {
		let instance = &self.instances[self.next];

		instance.set_volume(volume);
		instance.set_speed(speed);

		// If we've looped back to an instance before it stops playing,
		// rewind it and play again.
		if !self.single{
			instance.stop();
			instance.play();
		}

		self.next = (self.next + 1) % self.instances.len();
	}
}