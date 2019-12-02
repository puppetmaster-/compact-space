use specs::{Join, System, ReadStorage, WriteExpect};
use crate::components::{Position, Camera};
use crate::ressources::camera::{CameraRessource};

pub struct Sys {}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		ReadStorage<'a, Camera>,
		ReadStorage<'a, Position>,
		WriteExpect<'a, CameraRessource>,
	);

	fn run(&mut self, data : Self::SystemData) {
		let (cameras, positions, mut camera_ressource ) = data;
		for (_, pos) in (&cameras, &positions).join(){
			camera_ressource.center_on(pos.value).update()
		}
	}
}