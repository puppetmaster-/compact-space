use specs::{Join, System, WriteStorage};
use crate::components::{Rotation};

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = WriteStorage<'a, Rotation>;

	fn run(&mut self, data : Self::SystemData) {
		let mut rotatables = data;
		for rot in (&mut rotatables).join(){
			if rot.interval > 0.0{
				if rot.counterclockwise {
					rot.value -= rot.interval;
				}else{
					rot.value += rot.interval;
				}

				if !rot.always{
					rot.interval = 0.0;
				}
				if rot.value >= 360.0{
					rot.value -= 360.0;
				}
			}
		}
	}
}