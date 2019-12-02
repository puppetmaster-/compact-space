use specs::{System, WriteStorage};
use specs::join::Join;
use crate::components::{Timer};

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		WriteStorage<'a, Timer>
	);

	fn run(&mut self, data : Self::SystemData) {
		let mut timers = data;
		for time in (&mut timers).join(){
			if time.value > 0{
				time.value -= 1;
			}
		}
	}
}