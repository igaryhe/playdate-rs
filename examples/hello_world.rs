#![no_std]
#![no_main]

use playdate::*;
use playdate::graphics;
use anyhow::Result;
use serde::{Serialize, Deserialize};

const INIT_X: i32 = (400 - TEXT_WIDTH) / 2;
const INIT_Y: i32 = (240 - TEXT_HEIGHT) / 2;

const TEXT_WIDTH: i32 = 86;
const TEXT_HEIGHT: i32 = 16;

#[derive(Default, Copy, Clone)]
struct State {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

#[derive(Serialize, Deserialize)]
struct Des {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Game for State {
    fn init(playdate: &mut Playdate) -> Self {
        let font = graphics::Font::load("/System/Fonts/Asheville-Sans-14-Bold.pft").unwrap();
        playdate.graphics().set_font(font);
        Self {
            x: INIT_X,
            y: INIT_Y,
            dx: 1,
            dy: 2,
        }
    }
    fn update(&mut self, playdate: &mut Playdate) -> Result<()> {
        playdate.graphics().clear(graphics::Color::SolidColor(
            graphics::SolidColor::kColorWhite));
        playdate.graphics()
            .draw_text(
                "hello rust",
                graphics::StringEncoding::kASCIIEncoding,
                self.x,
                self.y,
            );
        self.x += self.dx;
        self.y += self.dy;
        if self.x < 0 || self.x > graphics::COLUMNS as i32 - TEXT_WIDTH {
            self.dx = -self.dx;
        }
        if self.y < 0 || self.y > graphics::ROWS as i32 - TEXT_HEIGHT {
            self.dy = -self.dy;
        }
        playdate.system().draw_fps(0, 0);
        Ok(())
    }
}

start_game!(State);
