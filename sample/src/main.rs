use std::rc::Rc;

use crate::components::Body;
use crate::components::Transform;
use legion::Entity;
use components::Shape;
use legion::query::*;
use rand::random;
use systems::{Kiss3DSystem, BodySystem};
use frameworky::*;

#[derive(Debug, Default)]
struct TestSystem {}
impl SimpleSystem for TestSystem
{
    fn execute(&mut self, context:&mut Context, event:&Rc<dyn Event>)
    {
        for i in 0..10 {
            let a = 0.1;
            let x = random::<f32>() * a;
            let y = 5.0 + i as f32;
            let z = random::<f32>() * a;
    
            let ball = (Transform::new(x, y, z), Body::default());
            context.world.push(ball);
        }
    }
}

fn main()
{
    let mut f :Frameworky = Frameworky::default();
    f.push_system(TestSystem::default());
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