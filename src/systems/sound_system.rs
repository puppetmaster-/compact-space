use specs::prelude::*;
use crate::components::PlaySound;

struct SoundRequest {
	sound_id: usize,
	vol: f32
}

pub struct SoundBuilder {
	requests : Vec<SoundRequest>,
}

impl SoundBuilder {
	pub fn new() -> SoundBuilder {
		SoundBuilder{
			requests : Vec::new(),
		}
	}

	pub fn request(&mut self, sound_id: usize, vol: f32) {
		self.requests.push(SoundRequest{ sound_id, vol });
	}
}

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, PlaySound>,
		WriteExpect<'a, SoundBuilder>
	);

	fn run(&mut self, data : Self::SystemData) {
		let (entities,
			mut playsounds,
			mut sound_builder,
		) = data;

		for sound_request in sound_builder.requests.iter() {
			entities.build_entity()
				.with(PlaySound{ id: sound_request.sound_id, vol: sound_request.vol }, &mut playsounds)
				.build();
		}
		sound_builder.requests.clear();
	}
}

