
use tetra::window;
use tetra::{Context, State};

pub trait Scene {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
}

#[allow(dead_code)]
pub enum Transition {
	None,
	Push(Box<dyn Scene>),
	Pop,
	Quit,
}

pub struct SceneManager {
	scenes: Vec<Box<dyn Scene>>,
}

impl SceneManager {
	pub fn new(initial_scene: Box<dyn Scene>) -> SceneManager {
		SceneManager {
			scenes: vec![initial_scene],
		}
	}
}

impl State for SceneManager {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result {
		match self.scenes.last_mut() {
			Some(active_scene) => match active_scene.update(ctx)? {
				Transition::None => {}
				Transition::Push(s) => {
					self.scenes.push(s);
				}
				Transition::Pop => {
					self.scenes.pop();
				}
				Transition::Quit => {
					window::quit(ctx)
				}
			},
			None => window::quit(ctx),
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		match self.scenes.last_mut() {
			Some(active_scene) => match active_scene.draw(ctx)? {
				Transition::None => {}
				Transition::Push(s) => {
					self.scenes.push(s);
				}
				Transition::Pop => {
					self.scenes.pop();
				}
				Transition::Quit => {
					window::quit(ctx)
				}
			},
			None => window::quit(ctx),
		}

		Ok(())
	}
}
