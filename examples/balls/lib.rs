use std::any::Any;
use components::{Body, Camera};
use components::Transform;
use components::Shape;
use hosts::kiss3d_host::Kiss3DHost;
use rand::random;
use systems::{BodySystem};
use frameworky::*;
use events::{KeyEvent};
use wasm_bindgen::prelude::*;
use legion::query::*;

#[derive(Debug, Default)]
struct ClickSystem
{
    spawn:bool
}

impl SimpleSystem for ClickSystem
{
    fn execute(&mut self, context:&mut Context, event:&dyn Any)
    {
        let key_event:Option<&KeyEvent> = event.downcast_ref::<KeyEvent>();
        if let Some(key) = key_event {
            println!("{}", key.key);

            // 10 = A, 13 = D, 32 = W, 28 = S

            <(&Camera, &mut Transform)>::query().for_each_mut(&mut context.world, |(c, t)| {
                let speed = 1.0;
                if key.key == 10
                {
                    // left
                    t.position.x -= speed;
                }
                else if key.key == 13
                {
                    // right
                    t.position.x += speed;
                }
            });
            

            if key.key == 76 // space
            {
                self.spawn = key.down;
            }
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


#[wasm_bindgen(start)]
pub fn start()
{
    let mut f :Frameworky = Frameworky::default();
    f.push_system(ClickSystem::default());
    f.push_system(BodySystem::default());

    
    let camera = (Transform::new(0.0, 20.0, 20.0), Camera::default());
    f.context.world.push(camera);

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
    
    Kiss3DHost::start(f, "Balls");
}