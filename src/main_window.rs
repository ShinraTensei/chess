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
            .window("Chess", 800, 800)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        Ok(Self {canvas, sdl_context})
    }

    pub fn start(&mut self) -> Result<(), String> {

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
            self.canvas.clear();

            //Display shit
            self.canvas.present();
        }
        Ok(())
    }
}