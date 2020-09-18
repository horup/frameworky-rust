use std::any::Any;
use crate::components::Body;
use crate::components::Transform;
use components::Shape;
use rand::random;
use systems::{Kiss3DSystem, BodySystem};
use frameworky::*;
use events::{KeyEvent};

#[derive(Debug, Default)]
struct ClickSystem
{
    spawn:bool
}

impl SimpleSystem for ClickSystem
{
    fn execute(&mut self, _context:&mut Context, event:&dyn Any)
    {
        let key_event = event.downcast_ref::<KeyEvent>();
        if let Some(key) = key_event {
            self.spawn = key.down;
        }
    }

    fn update(&mut self, context:&mut Context) 
    {
        if self.spawn {
            let a = 0.1;
            let x = random::<f32>() * a;
            let y = 10.0 as f32;
            let z = random::<f32>() * a;
    
            let ball = (Transform::new(x, y, z), Body::default());
            context.world.push(ball);
        }
    }
}

fn main()
{
    let mut f :Frameworky = Frameworky::default();
    f.push_system(ClickSystem::default());
    f.push_system(BodySystem::default());
    f.push_system(Kiss3DSystem::new("Sample!"));

    let plane = (Transform::new(0.0, 0.0, 0.0), Body::new(Shape::Plane));
    f.context.world.push(plane);
    
    for i in 0..10 {
        let a = 0.1;
        let x = random::<f32>() * a;
        let y = 5.0 + i as f32;
        let z = random::<f32>() * a;

        let ball = (Transform::new(x, y, z), Body::default());
        f.context.world.push(ball);
    }
    

    f.run();
}