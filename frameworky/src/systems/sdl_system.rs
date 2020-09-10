use crate::{SimpleSystem, Context};
use render::Canvas;
use pixels::Color;
use event::Event;
use video::Window;
use legion::World;
use sdl2::*;


#[derive(Default)]
pub struct SDLSystem
{
    sdl_context:Option<Sdl>,
    video_subsystem:Option<VideoSubsystem>,
    window:Option<Window>,
    canvas:Option<Canvas<Window>>,
    event_pump:Option<EventPump>
}

impl SimpleSystem for SDLSystem
{
    fn once(&mut self, context:&mut Context) {
        println!("Initializing SDL");
        self.sdl_context = Some(sdl2::init().unwrap());
        self.video_subsystem = Some(self.sdl_context.as_ref().unwrap().video().unwrap());
        self.window = Some(self.video_subsystem.as_ref().unwrap().
        window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap());

        self.event_pump = Some(self.sdl_context.as_ref().unwrap().event_pump().unwrap());
    /*    let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();
    
       /* canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();*/
        let mut event_pump = sdl_context.event_pump().unwrap();*/
    }

    fn update(&mut self, context:&mut Context) {
        let event_pump = self.event_pump.as_mut().unwrap();
        for e in event_pump.poll_iter()
        {
            match e {
                Event::Quit {..} |
                _ => {}
            }
        }
    }
}