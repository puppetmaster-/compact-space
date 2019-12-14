use tetra::{Context, graphics};
use tetra::graphics::{Texture, DrawParams, Text, Font};
use tetra::audio::Sound;
use crate::models::sound_pool::SoundPool;
use std::collections::HashMap;
use std::iter::Iterator;
use tetra::audio::Sound;
use crate::models::sound_pool::SoundPool;

type TextureHashmap = HashMap<TextureName, Texture>;
pub(crate) type SoundHashmap = HashMap<usize, SoundPool>;
type TextVec = Vec<Text>;

pub struct Assets{
	textures: TextureHashmap,
	texts: TextVec,
}
#[allow(dead_code)]
impl Assets{
	pub fn init(ctx: &mut Context) -> tetra::Result<Self>{
		Ok(Assets{
			textures: build_textures(ctx)?,
			texts: build_texts(ctx)?,
		})
	}

	pub fn draw(&self, ctx: &mut Context, asset_id: u16,draw_params: DrawParams){
		let name = TextureName::from_id(asset_id);
		graphics::draw(ctx,&self.textures[&name],draw_params);
	}

	pub fn draw_text(&self, ctx: &mut Context, index: usize, draw_params: DrawParams){
		graphics::draw(ctx,&self.texts[index],draw_params);
	}
	
	pub fn get_text_mut(&mut self) ->&mut Text{
		&mut self.texts[0]
	}

	pub fn build_sounds(&self, ctx: &mut Context) -> tetra::Result<SoundHashmap>{
		build_sounds(ctx)
	}

}

fn build_texts(_ctx: &mut Context) ->tetra::Result<TextVec>{
	let font = Font::default();
	Ok(vec![
		Text::new("", font, 32.0)
	])
}

fn build_textures(ctx: &mut Context) ->tetra::Result<TextureHashmap>{
	Ok([
		(TextureName::Arena, Texture::from_file_data(ctx, include_bytes!("../assets/art/arena_center.png"))?),
		(TextureName::ArenaText, Texture::from_file_data(ctx, include_bytes!("../assets/art/arena_text.png"))?),
		(TextureName::ArenaBorder, Texture::from_file_data(ctx, include_bytes!("../assets/art/arena_border.png"))?),
		(TextureName::NotFound, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_23.png"))?),
		(TextureName::Art0, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_00.png"))?),
		(TextureName::Art1, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_01.png"))?),
		(TextureName::Art2, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_02.png"))?),
		(TextureName::Art3, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_03.png"))?),
		(TextureName::Art4, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_04.png"))?),
		(TextureName::Art5, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_05.png"))?),
		(TextureName::Art6, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_06.png"))?),
		(TextureName::Art7, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_07.png"))?),
		(TextureName::Art8, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_08.png"))?),
		(TextureName::Art12, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_12.png"))?),
		(TextureName::Art16, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_16.png"))?),
		(TextureName::Art17, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_17.png"))?),
		(TextureName::Art18, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_18.png"))?),
		(TextureName::Art19, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_19.png"))?),
		(TextureName::Art20, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_20.png"))?),
		(TextureName::Astroid0, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_00.png"))?),
		(TextureName::Astroid1, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_01.png"))?),
		(TextureName::Astroid2, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_02.png"))?),
		(TextureName::Astroid3, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_03.png"))?),
		(TextureName::Astroid4, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_04.png"))?),
		(TextureName::Astroid5, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_05.png"))?),
		(TextureName::Astroid6, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_06.png"))?),
		(TextureName::Astroid7, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_07.png"))?),
		(TextureName::Astroid14, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_14.png"))?),
		(TextureName::Astroid15, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_15.png"))?),
		(TextureName::Astroid22, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_22.png"))?),
		(TextureName::Astroid23, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_23.png"))?),
		(TextureName::Astroid30, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_30.png"))?),
		(TextureName::Astroid31, Texture::from_file_data(ctx, include_bytes!("../assets/art/asteroid_31.png"))?),
		(TextureName::Enemy00, Texture::from_file_data(ctx, include_bytes!("../assets/art/enemy_00.png"))?),
		(TextureName::Enemy01, Texture::from_file_data(ctx, include_bytes!("../assets/art/enemy_01.png"))?),
		(TextureName::Enemy02, Texture::from_file_data(ctx, include_bytes!("../assets/art/enemy_02-xmas.png"))?),
		(TextureName::Parts00, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_08.png"))?),
		(TextureName::Parts01, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_09.png"))?),
		(TextureName::Parts02, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_10.png"))?),
		(TextureName::Parts03, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_11.png"))?),
		(TextureName::Parts04, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_12.png"))?),
		(TextureName::Parts05, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_16.png"))?),
		(TextureName::Parts06, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_17.png"))?),
		(TextureName::Parts07, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_18.png"))?),
		(TextureName::Parts08, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_19.png"))?),
		(TextureName::Parts09, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_20.png"))?),
		(TextureName::Parts10, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_24.png"))?),
		(TextureName::Parts11, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_25.png"))?),
		(TextureName::Parts12, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_26.png"))?),
		(TextureName::Parts13, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_27.png"))?),
		(TextureName::Parts14, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_28.png"))?),
		(TextureName::Parts15, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_29.png"))?),
		(TextureName::Parts16, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_30.png"))?),
		(TextureName::GuiGameOver, Texture::from_file_data(ctx, include_bytes!("../assets/art/gameover.png"))?),
		(TextureName::GuiStart, Texture::from_file_data(ctx, include_bytes!("../assets/art/start.png"))?),
		(TextureName::GuiPause, Texture::from_file_data(ctx, include_bytes!("../assets/art/pause.png"))?),
		].iter().cloned().collect()
	)
}

fn build_sounds(ctx: &mut Context) -> tetra::Result<SoundHashmap>{
	Ok([
		(0, SoundPool::new(ctx, Sound::from_file_data(include_bytes!("../assets/sound/explosion.wav")), 6)?),
		(1, SoundPool::new(ctx, Sound::from_file_data(include_bytes!("../assets/sound/explosion2.wav")), 6)?),
		(5, SoundPool::new(ctx, Sound::from_file_data(include_bytes!("../assets/sound/explosion1.wav")), 6)?),
		(2, SoundPool::new(ctx, Sound::from_file_data(include_bytes!("../assets/sound/pew.wav")), 6)?),
		(3, SoundPool::new(ctx, Sound::from_file_data(include_bytes!("../assets/sound/pew2.wav")), 6)?),
		(4, SoundPool::new(ctx, Sound::from_file_data(include_bytes!("../assets/sound/pew7.wav")), 6)?),
		(6, SoundPool::single(ctx, Sound::from_file_data(include_bytes!("../assets/sound/thrust.wav")))?),
		(7, SoundPool::new(ctx, Sound::from_file_data(include_bytes!("../assets/sound/metal-ping.wav")), 6)?),
		(8, SoundPool::new(ctx, Sound::from_file_data(include_bytes!("../assets/sound/explosion4.wav")), 6)?),
	].iter().cloned().collect()
	)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureName {
	Arena,
	ArenaText,
	ArenaBorder,
	NotFound,
	Art0,
	Art1,
	Art2,
	Art3,
	Art4,
	Art5,
	Art6,
	Art7,
	Art8,
	Art12,
	Art16,
	Art17,
	Art18,
	Art19,
	Art20,
	Astroid0,
	Astroid1,
	Astroid2,
	Astroid3,
	Astroid4,
	Astroid5,
	Astroid6,
	Astroid7,
	Astroid8,
	Astroid9,
	Astroid10,
	Astroid11,
	Astroid12,
	Astroid13,
	Astroid14,
	Astroid15,
	Astroid22,
	Astroid23,
	Astroid30,
	Astroid31,
	Parts00,
	Parts01,
	Parts02,
	Parts03,
	Parts04,
	Parts05,
	Parts06,
	Parts07,
	Parts08,
	Parts09,
	Parts10,
	Parts11,
	Parts12,
	Parts13,
	Parts14,
	Parts15,
	Parts16,
	Enemy00,
	Enemy01,
	Enemy02,
	GuiGameOver,
	GuiStart,
	GuiPause,
}

impl TextureName {
	pub fn from_id(n: u16)->TextureName{
		match n {
			0 => TextureName::Art0,
			1 => TextureName::Art1,
			2 => TextureName::Art2,
			3 => TextureName::Art3,
			4 => TextureName::Art4,
			5 => TextureName::Art5,
			6 => TextureName::Art6,
			7 => TextureName::Art7,
			8 => TextureName::Art8,
			12 => TextureName::Art12,
			16 =>  TextureName::Art16,
			17 =>  TextureName::Art17,
			18 =>  TextureName::Art18,
			19 =>  TextureName::Art19,
			20 =>  TextureName::Art20,
			100 => TextureName::Astroid0,
			101 => TextureName::Astroid1,
			102 => TextureName::Astroid2,
			103 => TextureName::Astroid3,
			104 => TextureName::Astroid4,
			105 => TextureName::Astroid5,
			106 => TextureName::Astroid6,
			107 => TextureName::Astroid7,
			108 => TextureName::Astroid8,
			109 => TextureName::Astroid9,
			110 => TextureName::Astroid10,
			111 => TextureName::Astroid11,
			112 => TextureName::Astroid12,
			113 => TextureName::Astroid13,
			114 => TextureName::Astroid14,
			115 => TextureName::Astroid15,
			122 => TextureName::Astroid22,
			123 => TextureName::Astroid23,
			130 => TextureName::Astroid30,
			131 => TextureName::Astroid31,
			200 => TextureName::Enemy00,
			201 => TextureName::Enemy01,
			202 => TextureName::Enemy02,
			300 => TextureName::Parts00,
			301 => TextureName::Parts01,
			302 => TextureName::Parts02,
			303 => TextureName::Parts03,
			304 => TextureName::Parts04,
			305 => TextureName::Parts05,
			306 => TextureName::Parts06,
			307 => TextureName::Parts07,
			308 => TextureName::Parts08,
			309 => TextureName::Parts09,
			310 => TextureName::Parts10,
			311 => TextureName::Parts11,
			312 => TextureName::Parts12,
			313 => TextureName::Parts13,
			314 => TextureName::Parts14,
			315 => TextureName::Parts15,
			316 => TextureName::Parts16,
			500 => TextureName::Arena,
			501 => TextureName::ArenaText,
			502 => TextureName::ArenaBorder,
			800 => TextureName::GuiGameOver,
			801 => TextureName::GuiStart,
			802 => TextureName::GuiPause,
			_ => TextureName::NotFound,
		}
	}
}

