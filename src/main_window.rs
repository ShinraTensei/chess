extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub struct Game {
    canvas: sdl2::render::WindowCanvas,
    sdl_context: sdl2::Sdl,
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("Chess", 768, 768)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Self {canvas, sdl_context})
    }

    pub fn start(&mut self) -> Result<(), String> {

        //Logic n shit
        let mut chess_board: [[bool; 8]; 8] = [[false; 8]; 8];
        let mut chess_board_copy = chess_board;
        let mut field = true;
        let mut tile = sdl2::rect::Rect::new(0, 0, 96, 96);
        let white = Color::RGB(255, 255, 255);
        let black = Color::RGB(0, 0, 0);

        for (x, row) in chess_board.iter_mut().enumerate() {
            for (y, _col) in row.iter_mut().enumerate() {
                field = !field;
                chess_board_copy[x][y] = field;
            }
            field = !field;
        }
        chess_board = chess_board_copy; //Fuck tha polis

        let mut event_pump = self.sdl_context.event_pump()?;
        'run: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'run,
                    _ => {}
                }
            }
            //Clear shit
            self.canvas.set_draw_color(Color::RGB(255, 0, 0));
            self.canvas.clear();

            //Draw board
            for y in 0..8usize {
                for x in 0..8usize {
                    tile.x = x as i32 + (x as i32 * tile.width() as i32);
                    tile.y = y as i32 + (y as i32 * tile.height() as i32);
                    match chess_board[y][x] {
                        true => {self.canvas.set_draw_color(black)}
                        false => {self.canvas.set_draw_color(white)}
                    }
                    self.canvas.fill_rect(tile)?;
                }
            }

            //Display shit
            self.canvas.present();
        }
        Ok(())
    }
}