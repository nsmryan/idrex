extern crate ggez;
extern crate rexpaint;

mod state;
mod gui;

use std::fs::File;
use std::io::BufReader;

use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::*;
use ggez::conf::{WindowMode, NumSamples, WindowSetup};

use rexpaint::*;

use crate::gui::Gui;
use crate::state::MainState;


pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("Font Indexer", "Font Indexing Tool");
    let cb = cb.window_mode(WindowMode::default().dimensions(1200.0, 800.0)
                                                 .resizable(true));
    let cb = cb.window_setup(WindowSetup::default().title("REXPaint Image Viewer"));
    let (ref mut ctx, event_loop) = &mut cb.build()?;

    let file = File::open("map.xp").expect("Could not open xp file");
    let mut buf_reader = BufReader::new(file);

    let tile_image = XpFile::read(&mut buf_reader).expect("Could not read xp file");
    let font_image = Image::new(ctx, "/rexpaint16x16.png").expect("Could not open font file");

    let state = &mut MainState::new(ctx, font_image, tile_image)?;

    return event::run(ctx, event_loop, state);
}
