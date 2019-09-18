use ggez::graphics;
use ggez::Context;

use gfx_core::{handle::RenderTargetView, memory::Typed};

use gfx_device_gl;

use imgui::*;
use imgui_gfx_renderer::*;

use std::time::Instant;

use crate::state::{Params, Info};


pub const GUI_HEIGHT: usize = 100;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
  pos: (i32, i32),
  pressed: (bool, bool, bool),
  wheel: f32,
}

pub struct Gui {
  pub imgui: imgui::Context,
  pub renderer: Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,
  last_frame: Instant,
  mouse_state: MouseState,
  show_popup: bool,
}

impl Gui {
  pub fn new(ctx: &mut Context) -> Self {
    // Create the imgui object
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    let (factory, gfx_device, _, _, _) = graphics::gfx_objects(ctx);

    // Shaders
    let shaders = {
      let version = gfx_device.get_info().shading_language;
      if version.is_embedded {
        if version.major >= 3 {
          Shaders::GlSlEs300
        } else {
          Shaders::GlSlEs100
        }
      } else if version.major >= 4 {
        Shaders::GlSl400
      } else if version.major >= 3 {
        Shaders::GlSl130
      } else {
        Shaders::GlSl110
      }
    };

    // Renderer
    let renderer = Renderer::init(&mut imgui, &mut *factory, shaders).unwrap();

    // Create instace
    Self {
      imgui,
      renderer,
      last_frame: Instant::now(),
      mouse_state: MouseState::default(),
      show_popup: false,
    }
  }

  pub fn render(&mut self, ctx: &mut Context, _params: &mut Params, info: &Info) {
    // Update mouse
    self.update_mouse();

    // Create new frame
    let now = Instant::now();
    let delta = now - self.last_frame;
    let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
    self.last_frame = now;

    let (w, h) = graphics::drawable_size(ctx);
    self.imgui.io_mut().display_size = [w, h];
    self.imgui.io_mut().display_framebuffer_scale = [1.0, 1.0];
    self.imgui.io_mut().delta_time = delta_s;

    let ui = self.imgui.frame();

    // Various ui things
    {
      // Window
      ui.window(im_str!("Index Selections"))
        .size([w, GUI_HEIGHT as f32], imgui::Condition::FirstUseEver)
        .position([0.0, h - GUI_HEIGHT as f32], imgui::Condition::FirstUseEver)
        .build(|| {
          if let Some(font_info) = info.font_info {
              let ch_index = font_info.x + font_info.y * 16;
              ui.text(format!("Font pos ({:2}, {:2}) {:3} (0x{:X}), char = {}", font_info.x, font_info.y, ch_index, ch_index, font_info.ch));
          } else {
              ui.new_line();
          }

          if let Some(map_info) = info.map_info {
              ui.text(format!("Map pos ({}, {})", map_info.x, map_info.y));
          } else {
              ui.new_line();
          }
        });
    }

    // Render
    let (factory, _, encoder, _, render_target) = graphics::gfx_objects(ctx);
    let draw_data = ui.render();
    self
      .renderer
      .render(
        &mut *factory,
        encoder,
        &mut RenderTargetView::new(render_target.clone()),
        draw_data,
      )
      .unwrap();
  }

  fn update_mouse(&mut self) {
    self.imgui.io_mut().mouse_pos = [self.mouse_state.pos.0 as f32, self.mouse_state.pos.1 as f32];

    self.imgui.io_mut().mouse_down = [
      self.mouse_state.pressed.0,
      self.mouse_state.pressed.1,
      self.mouse_state.pressed.2,
      false,
      false,
    ];

    self.imgui.io_mut().mouse_wheel = self.mouse_state.wheel;
    self.mouse_state.wheel = 0.0;
  }

  pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
    self.mouse_state.pos = (x as i32, y as i32);
  }

  pub fn update_mouse_down(&mut self, pressed: (bool, bool, bool)) {
    self.mouse_state.pressed = pressed;

    if pressed.0 {
      self.show_popup = false;
    }
  }

  pub fn update_key_down(&mut self) {
    self.show_popup = true;
  }
}
