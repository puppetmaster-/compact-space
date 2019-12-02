use specs::prelude::*;
use crate::components::{Input,Player};
use tetra::input::{self, Key, GamepadButton};
use tetra::Context;
use crate::ressources::{Gamestate, State};

#[allow(dead_code)]
pub struct Sys {}

pub fn update(world: &mut World, ctx: &mut Context){
	let mut game_state = world.write_resource::<Gamestate>();

	let mut input_storage = world.write_storage::<Input>();
	let players = world.read_storage::<Player>();
	let is_gamepad_connected = input::is_gamepad_connected(ctx, 0);
	for (input_entity,_) in (&mut input_storage, &players).join() {
		if game_state.state == State::Running{
			input_entity.right = input::is_key_down(ctx, Key::Right) || ( is_gamepad_connected && input::is_gamepad_button_down(ctx, 0, GamepadButton::Right ));
			input_entity.left = input::is_key_down(ctx, Key::Left) || ( is_gamepad_connected && input::is_gamepad_button_down(ctx, 0, GamepadButton::Left ));
			input_entity.throttle = input::is_key_down(ctx, Key::Up) || ( is_gamepad_connected && input::is_gamepad_button_down(ctx, 0, GamepadButton::X ));
			input_entity.brake = input::is_key_down(ctx, Key::Down) || ( is_gamepad_connected && input::is_gamepad_button_down(ctx, 0, GamepadButton::Down));
			input_entity.shoot = input::is_key_down(ctx, Key::Space) || ( is_gamepad_connected && input::is_gamepad_button_down(ctx, 0, GamepadButton::A ));
			input_entity.boost = input::is_key_down(ctx, Key::W) || ( is_gamepad_connected && input::is_gamepad_button_down(ctx, 0, GamepadButton::B ));
			if input::is_key_released(ctx, Key::Escape)
				|| ( is_gamepad_connected && input::is_gamepad_button_released(ctx, 0, GamepadButton::Start )){
				game_state.state = State::Pause;
			}
		}else if game_state.state == State::Dead {
			if input::is_key_released(ctx, Key::Space)
				|| ( is_gamepad_connected && input::is_gamepad_button_released(ctx, 0, GamepadButton::Start)){
				game_state.state = State::Init;
			}else if input::is_key_released(ctx, Key::Escape) || ( is_gamepad_connected && input::is_gamepad_button_released(ctx, 0, GamepadButton::Back)){
				game_state.state = State::Quit;
			}
			input_entity.right = false;
			input_entity.left = false;
			input_entity.throttle = false;
			input_entity.brake = false;
			input_entity.shoot = false;
			input_entity.boost = false;
		}else if game_state.state == State::Start {
			if input::is_key_released(ctx, Key::Space)
				|| ( is_gamepad_connected && input::is_gamepad_button_released(ctx, 0, GamepadButton::Start )){
				game_state.state = State::Init;
			}else if input::is_key_released(ctx, Key::Escape) || ( is_gamepad_connected && input::is_gamepad_button_released(ctx, 0, GamepadButton::Back)){
				game_state.state = State::Quit;
			}
		}else if game_state.state == State::Pause &&(input::is_key_released(ctx, Key::Escape)
			|| ( is_gamepad_connected && input::is_gamepad_button_released(ctx, 0, GamepadButton::Start ))){
			game_state.state = State::Running;
		}
	}
}

impl<'a> System<'a> for Sys {
	#[allow(clippy::type_complexity)]
	type SystemData = (
		ReadStorage<'a, Input>,
	);

	fn run(&mut self, _data: Self::SystemData) {
		unimplemented!()
	}
}

