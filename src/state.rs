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

        let mut width = 0;

        // Render game stuff
        for layer in self.tile_image.layers.iter() {
            //dbg!(layer.width, layer.height);
            for x in 0..layer.width {
                width = std::cmp::max(width, layer.width);

                for y in 0..layer.height {
                    let cell = layer.cells[y * layer.width + x];

                    let text = Text::new(format!("{}", cell.ch));

                    let pos = Point2::from([x as f32 * 16.0 * self.params.scale,
                                            y as f32 * 16.0 * self.params.scale]);

                    let chr_x = cell.ch % 16;
                    let chr_y = cell.ch / 16;
                    let src_rect =
                        Rect::new(chr_x as f32 / 16.0,
                                  chr_y as f32 / 16.0,
                                  1.0 / 16.0,
                                  1.0 / 16.0);
                    //dbg!(src_rect);
                    //dbg!(pos);
                    //dbg!(cell.ch);
                    let scaling = Vector2::from_slice(&[self.params.scale, self.params.scale]);
                    let params =
                        DrawParam::default().color(WHITE)
                                            .dest(pos)
                                            .src(src_rect)
                                            .scale(scaling);

                    //if cell.ch != ' ' as u32 {
                        ggez::graphics::draw(ctx, &self.font_image, params);
                    //}
                }
            }
        }

        let x_pos = width as f32 * self.params.scale * 16.0;
        let scaling = Vector2::from_slice(&[2.0, 2.0]);
        let params =
            DrawParam::default().dest(Point2::from([x_pos, 0.0]))
                                .scale(scaling);
        self.font_image.draw(ctx, params);


        // Render game ui
        {
            self.gui.render(ctx, &mut self.params);
        }

        ggez::graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.gui.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.gui.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.gui.update_mouse_down((false, false, false));
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::P => {
                //self.gui.open_popup();
                //self.gui.update_key_down(keycode, keymods);
            }
            _ => (),
        }
    }
}

