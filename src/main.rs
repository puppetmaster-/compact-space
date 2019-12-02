#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate specs_derive;

mod models;
mod scenes;
mod assets;
mod systems;
mod components;
mod ressources;
mod arena;
mod auxiliary;

use std::rc::Rc;
use tetra::ContextBuilder;
use std::cell::RefCell;

use crate::models::config::{load_config};
use crate::assets::Assets;
use crate::scenes::game::GameScene;
use crate::scenes::manager::SceneManager;

fn main() -> tetra::Result {
    let config = Rc::new(load_config(include_str!("../assets/configuration.ron")));
    let version = config.version();
    ContextBuilder::new(format!("{} v{}", config.titel, version).as_str(), config.window_width, config.window_height)
        .maximized(config.maximized)
        .fullscreen(config.fullscreen)
        .resizable(config.resizable)
        .scaling(config.scaling)
        .vsync(config.vsync)
        .show_mouse(config.show_mouse)
        .quit_on_escape(config.quit_on_escape)
        .build()?
        .run_with(|ctx| {
            let assets = Rc::new(RefCell::new(Assets::init(ctx)?));
            let scene = GameScene::new(ctx,config,assets)?;
            Ok(SceneManager::new(Box::new(scene)))
        })
}
