use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::*;
use ggez::{Context, GameResult};

use mint::Point2;
use mint::Vector2;

use rexpaint::*;

use crate::gui::Gui;


pub struct Params {
    pub scale: f32,
}

impl Default for Params {
    fn default() -> Params {
        Params {
            scale: 0.52,
        }
    }
}

pub struct FontInfo {
    x: i32,
    y: i32,
    ch: char,
}

pub struct MapInfo {
    x: i32,
    y: i32,
    ch: char,
    // image? drawparams?
}

pub struct Info {
    pub font_info: Option<FontInfo>,
    pub map_info: Option<MapInfo>,
}

impl Default for Info {
    fn default() -> Info {
        Info {
            font_info: None,
            map_info: None,
        }
    }
}

pub struct MainState {
    pub gui: Gui,
    pub font_image: Image,
    pub tile_image: XpFile,
    pub params: Params,
    pub info: Info,
}

impl MainState {
    pub fn new(mut ctx: &mut Context,
           font_image: Image,
           tile_image: XpFile) -> GameResult<MainState> {
        let gui = Gui::new(&mut ctx);
        let s = MainState {
            gui,
            font_image,
            tile_image,
            params: Params::default(),
            info: Info::default(),
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let background = ggez::graphics::Color::new(255.0 / 255.0, 140.0 / 255.0, 0.0, 1.0);
        ggez::graphics::clear(ctx, background);


        /* Calculate dimensions of each component of the screen */
        let screen_coords = ggez::graphics::screen_coordinates(ctx);

        let mut map_disp = screen_coords;
        map_disp.scale(0.5, 1.0);
        let map_disp_width = map_disp.w;
        let map_disp_height = map_disp.h;
        let map_width = self.tile_image.layers[0].width as f32 * 16.0;
        let map_height = self.tile_image.layers[0].height as f32 * 16.0;
        let map_scaler;
        if (map_disp.w / map_width) <= map_disp.h {
            map_scaler = map_disp.w / map_width;
        } else {
            map_scaler = map_disp.h / map_height;
        }
        map_disp.scale(map_scaler, map_scaler);
        map_disp.move_to([(map_disp_width - map_width * map_scaler) / 2.0,
                          (map_disp_height -  map_height * map_scaler) / 2.0,]);

        let mut font_disp = screen_coords;
        font_disp.scale(0.5, 0.5);
        font_disp.move_to([screen_coords.w / 2.0, 0.0]);
                              
        let mut char_disp = screen_coords;
        char_disp.scale(0.5, 0.5);
        char_disp.move_to([screen_coords.w / 2.0, screen_coords.h / 2.0]);

        // character to use in character display
        let mut char_to_display: Option<char> = None;
        let mouse_pos = ggez::input::mouse::position();

        // draw map display
        {
            // Render game stuff
            for layer in self.tile_image.layers.iter() {
                for x in 0..layer.width {
                    for y in 0..layer.height {
                        let cell = layer.cells[y * layer.width + x];

                        let pos = Point2::from([x as f32 * 16.0 * self.params.scale,
                                                y as f32 * 16.0 * self.params.scale]);

                        let src_rect =
                            Rect::new((cell.ch % 16) as f32 / 16.0,
                                      (cell.ch / 16) as f32 / 16.0,
                                      1.0 / 16.0,
                                      1.0 / 16.0);
                        let params =
                            DrawParam::default().color(WHITE)
                                                .dest([map_disp.x + x as f32 * map_scaler * 16.0,
                                                       map_disp.y + y as f32 * map_scaler * 16.0])
                                                .src(src_rect)
                                                .scale([map_scaler, map_scaler]);

                        //if cell.ch != ' ' as u32 {
                            ggez::graphics::draw(ctx, &self.font_image, params)?;
                        //}

                        // TODO unfinished- need variables for ch_*
                        if map_disp.contains(mouse_pos) &&
                           (mouse_pos.x >= pos.x && mouse_pos.x < (pos.x + ch_width)) &&
                           (mouse_pos.y >= pos.y && mouse_pos.y < (pos.y + ch_height)) {
                                char_to_display = cells.ch;
                            }
                        }
                    }
                }
            }
        }

        // draw font display
        { 
            let params =
                DrawParam::default().dest([font_disp.x, font_disp.y])
                                    // TODO the 256.0 should depend on the font image dimensions
                                    .scale([font_disp.w / 256.0, font_disp.h / 256.0]);
            self.font_image.draw(ctx, params)?;

            // TODO highlight character under cursor, or character in map under cursor
        }

        // draw character display
        {
            // TODO display character in font or map
            // TODO display index in decimal and hex, and the ascii character if any
        }

        // Render game ui
        {
            self.gui.render(ctx, &mut self.params);
        }

        ggez::graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self,
                          _ctx: &mut Context,
                          x: f32,
                          y: f32,
                          _dx: f32,
                          _dy: f32) {
        self.gui.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(&mut self,
                               _ctx: &mut Context,
                               button: MouseButton,
                               _x: f32,
                               _y: f32) {
        self.gui.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(&mut self,
                             _ctx: &mut Context,
                             _button: MouseButton,
                             _x: f32,
                             _y: f32) {
        self.gui.update_mouse_down((false, false, false));
    }

    fn key_down_event(&mut self,
                      _ctx: &mut Context,
                      keycode: KeyCode,
                      keymods: KeyMods,
                      _repeat: bool) {
        match keycode {
            KeyCode::P => {
                //self.gui.open_popup();
                //self.gui.update_key_down(keycode, keymods);
            }
            _ => (),
        }
    }
}

