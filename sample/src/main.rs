use systems::SDLSystem;
use frameworky::*;

#[derive(Debug, Default)]
struct TestSystem {}
impl SimpleSystem for TestSystem
{
    fn once(&mut self, context:&mut Context) {
        
    }
    fn update(&mut self, context:&mut Context) {
    }
}

fn main()
{
    let mut f :Frameworky = Frameworky::default();
    f.push_system(TestSystem::default());
    f.push_system(SDLSystem::new("Sample", 800, 640));
    f.run();
}

/*
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    /*let mut f = Frameworky::default();
    {
        for i in 0..10
        {
            let cs = (
                Transform::new(i as f32, 0.0, 0.0), 
                Body::default()
            );
            f.world.push(cs);
        }

        let mut q = <&Transform>::query();
        for p in q.iter(&f.world)
        {
            println!("{:?}", p);
        }
    }*/
}
*/