#![no_std]

use core::ptr;
use playdate::*;
use playdate::graphics;

const INIT_X: i32 = (400 - TEXT_WIDTH) / 2;
const INIT_Y: i32 = (240 - TEXT_HEIGHT) / 2;

const TEXT_WIDTH: i32 = 86;
const TEXT_HEIGHT: i32 = 16;

#[derive(Default)]
struct State {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    font: Option<graphics::Font>,
}

impl Game for State {
    fn init(&mut self, playdate: &mut Playdate) {
        self.x = INIT_X;
        self.y = INIT_Y;
        self.dx = 1;
        self.dy = 2;
        self.font = playdate.graphics()
            .load_font("/System/Fonts/Asheville-Sans-14-Bold.pft").ok();
    }
    fn update(&mut self, playdate: &mut Playdate) {
        playdate.graphics().clear(graphics::Color::SolidColor(
            graphics::SolidColor::kColorWhite,
        ));
        playdate.graphics()
            .draw_text(
                &self.font.unwrap(),
                ptr::null_mut(),
                ptr::null_mut(),
                "Hello World",
                graphics::PDStringEncoding::kASCIIEncoding,
                self.x,
                self.y,
                graphics::BitmapDrawMode::kDrawModeCopy,
                0,
                graphics::Rect::default(),
            )
            .unwrap();
        self.x += self.dx;
        self.y += self.dy;
        if self.x < 0 || self.x > graphics::COLUMNS as i32 - TEXT_WIDTH {
            self.dx = -self.dx;
        }
        if self.y < 0 || self.y > graphics::ROWS as i32 - TEXT_HEIGHT {
            self.dy = -self.dy;
        }
    }
}

start_game!(State, State {x:0, y:0, dx:0, dy: 0, font: None});