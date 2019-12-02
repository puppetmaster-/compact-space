use ron::de::{from_str};

use serde::{Serialize, Deserialize};

use tetra::graphics::{Color,ScreenScaling};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	// Game configuration
	pub titel: String,
	#[serde(with = "ColorDef")]
	pub clear_color: Color,
	// Window settings
	pub window_width: i32,
	pub window_height: i32,
	pub maximized: bool,
	pub fullscreen: bool,
	pub resizable: bool,
	#[serde(with = "ScreenScalingDef")]
	pub scaling: ScreenScaling,
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

#[derive(Serialize, Deserialize)]
#[serde(remote = "ScreenScaling")]
pub enum ScreenScalingDef {
	None,
	Stretch,
	ShowAll,
	ShowAllPixelPerfect,
	Crop,
	CropPixelPerfect,
	Resize,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Color")]
struct ColorDef {
	r: f32,
	g: f32,
	b: f32,
	a: f32,
}
