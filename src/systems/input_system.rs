use specs::prelude::*;
use crate::components::{Input,Player};
use tetra::input::{self, Key, GamepadButton};
use tetra::{Context, Event};
use crate::ressources::{Gamestate, State};

#[allow(dead_code)]
pub struct Sys {}

pub fn update(world: &mut World, ctx: &mut Context, _event: Event){
	let mut game_state = world.write_resource::<Gamestate>();

	let mut input_storage = world.write_storage::<Input>();
	let players = world.read_storage::<Player>();
	for (input_entity,_) in (&mut input_storage, &players).join() {
		if game_state.state == State::Running{
			input_entity.right = input::is_key_down(ctx, Key::Right) || input::is_gamepad_button_down(ctx, 0, GamepadButton::Right );
			input_entity.left = input::is_key_down(ctx, Key::Left) || input::is_gamepad_button_down(ctx, 0, GamepadButton::Left );
			input_entity.throttle = input::is_key_down(ctx, Key::Up) || input::is_gamepad_button_down(ctx, 0, GamepadButton::X );
			input_entity.brake = input::is_key_down(ctx, Key::Down) || input::is_gamepad_button_down(ctx, 0, GamepadButton::Down);
			input_entity.shoot = input::is_key_down(ctx, Key::Space) || input::is_gamepad_button_down(ctx, 0, GamepadButton::A );
			input_entity.boost = input::is_key_down(ctx, Key::W) || input::is_gamepad_button_down(ctx, 0, GamepadButton::B );
			if input::is_key_released(ctx, Key::Escape) || input::is_gamepad_button_released(ctx, 0, GamepadButton::Start ){
				game_state.state = State::Pause;
			}
		}else if game_state.state == State::Dead {
			if input::is_key_released(ctx, Key::Enter) || input::is_gamepad_button_released(ctx, 0, GamepadButton::Start){
				game_state.state = State::Init;
			}else if input::is_key_released(ctx, Key::Escape) || input::is_gamepad_button_released(ctx, 0, GamepadButton::Back){
				game_state.state = State::Quit;
			}
			input_entity.right = false;
			input_entity.left = false;
			input_entity.throttle = false;
			input_entity.brake = false;
			input_entity.shoot = false;
			input_entity.boost = false;
		}else if game_state.state == State::Start {
			if input::is_key_released(ctx, Key::Space) || input::is_gamepad_button_released(ctx, 0, GamepadButton::Start ){
				game_state.state = State::Init;
			}else if input::is_key_released(ctx, Key::Escape) || input::is_gamepad_button_released(ctx, 0, GamepadButton::Back){
				game_state.state = State::Quit;
			}
		}else if game_state.state == State::Pause {
			if input::is_key_released(ctx, Key::Space) || input::is_gamepad_button_released(ctx, 0, GamepadButton::Start){
				game_state.state = State::Running;
			}else if input::is_key_released(ctx, Key::Escape) || input::is_gamepad_button_released(ctx, 0, GamepadButton::Back){
				game_state.state = State::Quit;
			}
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

