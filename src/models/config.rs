use ron::de::{from_str};

use serde::{Serialize, Deserialize};

use tetra::graphics::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	// Game configuration
	pub titel: String,
	pub clear_color: Color,
	// Window settings
	pub window_width: i32,
	pub window_height: i32,
	pub maximized: bool,
	pub fullscreen: bool,
	pub resizable: bool,
	// Tetra settings
	pub show_mouse: bool,
	pub vsync: bool,
	pub quit_on_escape: bool,
	// Game Music
	pub master_volume: f32,
}

impl Config {

	pub fn version(&self) -> String{
		env!("CARGO_PKG_VERSION").to_owned()
	}
}

pub fn load_config(path: &str) -> Config{
	match from_str(path){
		Ok(config) => config,
		Err(error) => {
			println!("Failed to load config: {}", error);
			std::process::exit(1);
		}
	}
}
