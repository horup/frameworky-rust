use crate::{SimpleSystem, Context};
use render::Canvas;
use event::Event;
use video::Window;
use sdl2::*;

pub struct SDLSystem
{
    sdl_context:Sdl,
    video_subsystem:VideoSubsystem,
    canvas:Canvas<Window>,
    event_pump:EventPump
}

impl SDLSystem
{
    pub fn new(title:&str, width:u32, height:u32) -> Self
    {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window(title, width, height).build().unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        return SDLSystem {
            sdl_context:sdl_context,
            video_subsystem:video_subsystem,
            canvas:canvas,
            event_pump:event_pump
        }
    }
}

impl SimpleSystem for SDLSystem
{
    fn update(&mut self, context:&mut Context) {
        for e in self.event_pump.poll_iter()
        {
            match e {
                Event::Quit {..} => {context.running = false},
                _ => {}
            }
        }
    }
}