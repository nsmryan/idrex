use std::iter::*;
use std::slice::Iter;

use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::*;
use ggez::{Context, GameResult};

use mint::Point2;

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

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FontInfo {
    pub x: i32,
    pub y: i32,
    pub ch: char,
}

impl Default for FontInfo {
    fn default() -> FontInfo {
        FontInfo {
            x: 0,
            y: 0,
            ch: ' ',
        }
    }
}

impl FontInfo {
    pub fn new(x: i32, y: i32, ch: char) -> FontInfo {
        FontInfo {
            x,
            y,
            ch,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MapInfo {
    pub x: usize,
    pub y: usize,
    pub ch: char,
}

impl Default for MapInfo {
    fn default() -> MapInfo {
        MapInfo {
            x: 0,
            y: 0,
            ch: ' ',
        }
    }
}

impl MapInfo {
    pub fn new(ch: char, x: usize, y: usize) -> MapInfo {
        MapInfo {
            ch,
            x,
            y,
        }
    }
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
    pub map_disp: Rect,
    pub font_disp: Rect,
    pub char_disp: Rect,
    pub map_scaler: f32,
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
            map_disp: Rect::default(),
            font_disp: Rect::default(),
            char_disp: Rect::default(),
            map_scaler: 0.0,
        };
        Ok(s)
    }
}

pub fn map_ch_dims(map_disp: Rect, map: &XpLayer) -> (f32, f32) {
    return ((map_disp.w / map.width as f32),
            (map_disp.h / map.height as f32));
}

pub fn map_pos_to_screen(map_disp: Rect, x: usize, y: usize, map_scaler: f32) -> Point2<f32> {
    let pos = Point2::from([map_disp.x + x as f32 * map_scaler,
                            map_disp.y + y as f32 * map_scaler]);

    return pos;
}

pub fn calc_map_scaler(map_disp: Rect, map_width: usize, map_height: usize) -> f32 {
    let map_scaler;

    let width_scale = map_disp.w / map_width as f32;
    let height_scale = map_disp.h / map_height as f32;

    let width_scale_error = map_disp.h - width_scale * map_height as f32;
    let height_scale_error = map_disp.w - height_scale * map_width as f32;

    println!("");
    dbg!(map_disp.w, map_disp.h);
    dbg!(map_width, map_height);
    dbg!(width_scale, height_scale);
    dbg!(width_scale_error, height_scale_error);
    if width_scale_error >= 0.0 && width_scale_error < height_scale_error {
        map_scaler = width_scale;
    } else if height_scale_error < 0.0 {
        map_scaler = width_scale;
    } else {
        map_scaler = height_scale;
    }

    return map_scaler;
}

pub fn hightlight_square(ctx: &mut Context, pos: Point2<f32>, width: f32, height: f32, color: Color) -> GameResult<()> {
    let highlight =
        Mesh::new_rectangle(ctx,
                            DrawMode::Stroke(StrokeOptions::default()),
                            Rect::new(pos.x - 1.0,
                                      pos.y - 1.0,
                                      width + 2.0,
                                      height + 2.0),
                            color)?;
    highlight.draw(ctx, DrawParam::default())?;

    Ok(())
}

pub fn calc_map_coords(screen_coords: Rect, width_cells: usize, height_cells: usize) -> (Rect, f32) {
    let mut map_disp = screen_coords;

    // split screen into two section, the right section being the map
    map_disp.scale(0.5, 1.0);

    // save the current dimensions as the full available area
    let full_map_disp = map_disp;

    let map_scaler = calc_map_scaler(map_disp, width_cells, height_cells);

    let used_map_width = map_scaler * width_cells as f32;
    let used_map_height = map_scaler * height_cells as f32;

    let x_margin = full_map_disp.w - used_map_width;
    let y_margin = full_map_disp.h - used_map_height;

    map_disp.scale(used_map_width / full_map_disp.w, used_map_height / full_map_disp.h);

    map_disp.move_to([x_margin / 2.0, y_margin / 2.0]);

    return (map_disp, map_scaler);
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mouse_pos = ggez::input::mouse::position(ctx);

        /* Calculate dimensions of each component of the screen */
        let screen_coords = ggez::graphics::screen_coordinates(ctx);
        let pair =
            calc_map_coords(screen_coords,
                            self.tile_image.layers[0].width,
                            self.tile_image.layers[0].height);
        self.map_disp = pair.0;
        self.map_scaler = pair.1;

        self.font_disp = screen_coords;
        self.font_disp.scale(0.5, 0.5);
        self.font_disp.move_to([screen_coords.w / 2.0, 0.0]);
                              
        self.char_disp = screen_coords;
        self.char_disp.scale(0.5, 0.5);
        self.char_disp.move_to([screen_coords.w / 2.0, screen_coords.h / 2.0]);

        // Font Info
        self.info.font_info = None;

        let ch_width = self.font_disp.w as f32 / 16.0;
        let ch_height = self.font_disp.h as f32 / 16.0;
        if self.font_disp.contains(mouse_pos) {
            // get character under cursor
            let x = (((mouse_pos.x - self.font_disp.x) as f32 / self.font_disp.w as f32) * 16.0) as i32;
            let y = (((mouse_pos.y - self.font_disp.y) as f32 / self.font_disp.h as f32) * 16.0) as i32;
            let ch = std::char::from_u32((x + y * 16) as u32).unwrap();
            self.info.font_info = Some(FontInfo::new(x, y, ch));
        }

        // Map Info
        self.info.map_info = None;

        for layer in self.tile_image.layers.iter() {
            let cell_iter = layer.cells.iter()
                                 .enumerate()
                                 .map(|(index, cell)|
                                        (index % layer.width,
                                         index / layer.width,
                                         std::char::from_u32(cell.ch).unwrap())
                                      );
            for (x, y, ch) in cell_iter {
                let pos = map_pos_to_screen(self.map_disp, x, y, self.map_scaler);

                if (mouse_pos.x >= pos.x && mouse_pos.x < (pos.x + self.map_scaler)) &&
                   (mouse_pos.y >= pos.y && mouse_pos.y < (pos.y + self.map_scaler)) {
                    self.info.map_info =
                        Some(MapInfo::new(ch, x, y));

                    self.info.font_info = Some(FontInfo::new(x as i32, y as i32, ch));
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let background = ggez::graphics::Color::new(255.0 / 255.0, 140.0 / 255.0, 0.0, 1.0);
        let highlight_color = Color::new(255.0 / 256.0, 140.0 / 256.0, 0.0, 1.0);

        ggez::graphics::clear(ctx, background);

        let map_width = self.tile_image.layers[0].width;
        let map_height = self.tile_image.layers[0].height;

        // character to use in character display
        let mouse_pos = ggez::input::mouse::position(ctx);

        // draw font display
        { 
            let params =
                DrawParam::default().dest([self.font_disp.x, self.font_disp.y])
                                    // TODO the 256.0 should depend on the font image dimensions
                                    .scale([self.font_disp.w / 256.0, self.font_disp.h / 256.0]);
            self.font_image.draw(ctx, params)?;

            if let Some(font_info) = self.info.font_info {
                let ch_width = self.font_disp.w as f32 / 16.0;
                let ch_height = self.font_disp.h as f32 / 16.0;
                let x = (((mouse_pos.x - self.font_disp.x) as f32 / self.font_disp.w as f32) * 16.0) as i32;
                let y = (((mouse_pos.y - self.font_disp.y) as f32 / self.font_disp.h as f32) * 16.0) as i32;
                let pos = Point2::from([self.font_disp.x + x as f32 * ch_width - 1.0,
                                        self.font_disp.y + y as f32 * ch_height - 1.0]);
                hightlight_square(ctx, pos, ch_width, ch_height, highlight_color)?;
            }
        }

        // draw map display
        {
            dbg!(self.map_scaler);
            // Render game stuff
            for layer in self.tile_image.layers.iter() {
                let tile_iter =
                    layer.cells.iter().enumerate().map(
                        |(index, cell)| (index % layer.width, index / layer.width, cell)
                     );

                for (x, y, cell) in tile_iter {
                    let ch = std::char::from_u32(cell.ch).unwrap();

                    let src_rect =
                        Rect::new((cell.ch % 16) as f32 / 16.0,
                                  (cell.ch / 16) as f32 / 16.0,
                                  1.0 / 16.0,
                                  1.0 / 16.0);

                    let pos = map_pos_to_screen(self.map_disp, x, y, self.map_scaler);

                    let params =
                        DrawParam::default().color(WHITE)
                                            .dest(pos)
                                            .src(src_rect)
                                            .scale([self.map_scaler / 16.0, self.map_scaler / 16.0]);

                    ggez::graphics::draw(ctx, &self.font_image, params)?;
                }
            }

            // Draw highlight on font square
            if let Some(font_info) = self.info.font_info {
                for layer in self.tile_image.layers.iter() {
                    let (ch_width, ch_height) = map_ch_dims(self.map_disp, layer);

                    for x in 0..layer.width {
                        for y in 0..layer.height {
                            let cell = layer.cells[y * layer.width + x];
                            let ch = std::char::from_u32(cell.ch).unwrap();
                            let pos = Point2::from([self.map_disp.x + x as f32 * ch_width,
                                                    self.map_disp.y + y as f32 * ch_height]);
                            if ch == font_info.ch {
                                hightlight_square(ctx, pos, self.map_scaler, self.map_scaler, highlight_color)?;
                            }
                        }
                    }
                }

                let ch_width = self.font_disp.w as f32 / 16.0;
                let ch_height = self.font_disp.h as f32 / 16.0;
                let x = font_info.ch as usize % 16;
                let y = font_info.ch as usize / 16;
                let pos = Point2::from([self.font_disp.x + x as f32 * ch_width - 1.0,
                                        self.font_disp.y + y as f32 * ch_height - 1.0]);
                hightlight_square(ctx, pos, ch_width, ch_height, highlight_color)?;
            }
        }

        // draw character display
        {
            // TODO display character in font or map
            // TODO display index in decimal and hex, and the ascii character if any
            if let Some(font_info) = self.info.font_info {
                let ch = font_info.ch as usize;
                let src_rect =
                    Rect::new((ch % 16) as f32 / 16.0,
                              (ch / 16) as f32 / 16.0,
                              1.0 / 16.0,
                              1.0 / 16.0);

                let pos = Point2::from([self.char_disp.x, self.char_disp.y]);

                let params =
                    DrawParam::default().color(WHITE)
                                        .dest(pos)
                                        .src(src_rect)
                                        .scale([self.char_disp.w / 16.0, self.char_disp.h / 16.0]);

                ggez::graphics::draw(ctx, &self.font_image, params)?;
            }
        }

        // Render game ui
        {
            self.gui.render(ctx, &mut self.params, &self.info);
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

