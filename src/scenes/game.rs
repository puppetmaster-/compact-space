use std::rc::Rc;
use std::cell::{RefCell, Ref};
use rand::prelude::*;
use crate::models::config::Config;
use crate::assets::{Assets,SoundHashmap};
use tetra::{Context, graphics, audio, window, Event, time};
use specs::prelude::*;
use tetra::math::Vec2;
use crate::components::{Position, Renderable, Rotation, Hidden, Scaleable, PlaySound};
use crate::systems::*;
use tetra::graphics::DrawParams;
use crate::ressources::{camera::CameraRessource, Gamestate, State};
use crate::scenes::manager::{Transition, Scene};
use crate::{arena, components, ressources};
use crate::auxiliary::*;
use std::time::{Instant, Duration};
use tetra::audio::{SoundInstance, Sound};

#[allow(dead_code)]
pub struct GameScene {
	world: World,
	alive: Instant,
	alive_sum: Duration,
	measurement_running: bool,
	dispatcher: Dispatcher<'static, 'static>,
	config: Rc<Config>,
	assets: Rc<RefCell<Assets>>,
	background_music_instance: SoundInstance,
	sounds: SoundHashmap,
	randomizer: ThreadRng,
	debug: bool,
}

impl GameScene {
	pub fn new(ctx: &mut Context, config: Rc<Config>, assets: Rc<RefCell<Assets>>) -> tetra::Result<GameScene> {
		let mut world= World::new();
		components::register(&mut world);
		arena::create(&mut world);
		ressources::insert(ctx, &mut world);
		audio::set_master_volume(ctx, config.master_volume);
		let background_music = Sound::from_file_data(include_bytes!("../../assets/music/Star Flow.mp3"));
		let background_music_instance = background_music.spawn(ctx)?;
		background_music_instance.set_repeating(true);
		background_music_instance.play();
		background_music_instance.set_volume(0.2);
		let sounds = assets.borrow().build_sounds(ctx)?;

		Ok(GameScene {
			world,
			alive: Instant::now(),
			alive_sum: Duration::new(0,0),
			measurement_running: false,
			dispatcher: create_dispatcher(),
			assets,
			config,
			background_music_instance,
			sounds,
			randomizer: rand::thread_rng(),
			debug: false,
		})
	}
}

impl Scene for GameScene {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
		// automatic
		self.dispatcher.dispatch(&self.world);
		if self.debug{
			self.assets.borrow_mut().get_text_mut(1).set_content(format!("FPS: {:?}", (time::get_fps(ctx) + 0.1) as i32));
		}

		// manual
		lifetime_system::cull_deads(&mut self.world);

		if self.world.read_resource::<Gamestate>().state == State::Init{
			self.world.write_resource::<Gamestate>().state = State::Running;
			{
				arena::reset(&mut self.world, SeedableRng::from_seed(SEED));
				self.alive = Instant::now();
				self.alive_sum = Duration::new(0,0);
				self.measurement_running = true;
			}
		}
		if self.world.write_resource::<Gamestate>().state != State::Running && self.measurement_running{
			self.alive_sum += self.alive.elapsed();
			self.measurement_running = false;
			let min = (self.alive_sum / 60).as_secs();
			let sec = self.alive_sum - Duration::from_secs((min*60) as u64);
			self.assets.borrow_mut().get_text_mut(0).set_content(format!("{:?} min {:.2} sec", min, sec.as_secs_f32()));
		}
		if self.world.read_resource::<Gamestate>().state == State::Quit{
			window::quit(ctx);
		}

		// play sound
		let mut played = vec![];
		{
			let play_sounds = self.world.read_storage::<PlaySound>();
			let entities = self.world.entities();
			for (entity, play_sound) in (&entities, &play_sounds).join(){
				self.sounds.get_mut(&play_sound.id).expect("no sound").play(play_sound.vol, self.randomizer.gen_range(0.5, 0.6));
				played.push(entity);
			}
		}
		self.world.delete_entities(&played).expect("can't delete played sound");

		self.world.maintain();

		Ok(Transition::None)
	}

	fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
		graphics::clear(ctx, self.config.clear_color);

		draw_entities(ctx, &self.world, self.assets.borrow());

		let camera = self.world.read_resource::<CameraRessource>();
		match self.world.read_resource::<Gamestate>().state{
			State::Start => {self.assets.borrow().draw(ctx, 801, DrawParams::new()
				.position(camera.window_half)
				.origin(Vec2::new(128.0,32.0))
			);},
			State::Dead => {self.assets.borrow().draw(ctx, 800, DrawParams::new()
				.position(camera.window_half)
				.origin(Vec2::new(128.0,32.0))
			);
				self.assets.borrow().draw_text(ctx, 0, DrawParams::new()
					.position(camera.window_half-Vec2::new(120.0,-40.0))
				);},
			State::Pause => {self.assets.borrow().draw(ctx, 802, DrawParams::new()
				.position(camera.window_half)
				.origin(Vec2::new(128.0,32.0))
			);},
			_ => {}
		}

		if self.debug{
			self.assets.borrow().draw_text(ctx, 1, DrawParams::new()
				.position(Vec2::new(20.0,20.0))
			);
		}
		Ok(Transition::None)
	}

	fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result<Transition> {
		if let Event::FocusLost = event{
			if self.world.read_resource::<Gamestate>().state != State::Dead && self.world.read_resource::<Gamestate>().state != State::Start {
				self.world.write_resource::<Gamestate>().state = State::Pause;
			}
		}
		input_system::update(&mut self.world, ctx, event);

		Ok(Transition::None)
	}
}


fn draw_entities(ctx: &mut Context,world: &World, assets: Ref<Assets>){
	let camera = world.read_resource::<CameraRessource>();

	let positions = world.read_storage::<Position>();
	let renderables = world.read_storage::<Renderable>();
	let entities = world.entities();
	let rotations = world.read_storage::<Rotation>();
	let scaleable = world.read_storage::<Scaleable>();
	let hidden = world.read_storage::<Hidden>();

	let mut data = (&positions, &renderables, &entities, !&hidden).par_join().collect::<Vec<_>>();
	data.sort_by(|&a, &b|{
		b.1.render_order.cmp(&a.1.render_order)
			.then(b.1.texture_id.cmp(&a.1.texture_id))
			.then(b.0.value.y.partial_cmp(&a.0.value.y).unwrap())
	} );

	for (pos, render, entity, _) in data.iter().rev() {
		let rot = match rotations.get(*entity){
			Some(r) => r.value as f32,
			_ => 0.0
		};
		let scale = match scaleable.get(*entity){
			Some(s) => s.value,
			_ => Vec2::one(),
		};
		let draw_params = DrawParams::new()
			.position(pos.value - camera.offset)
			.color(render.color)
			.rotation(degrees_to_radians(rot))
			.scale(scale)
			.origin(render.origin);
		assets.draw(ctx, render.texture_id, draw_params);
	}
}