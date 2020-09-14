use crate::components::Body;
use crate::components::Transform;
use legion::Entity;
use legion::query::*;
use rand::random;
use systems::{Kiss3DSystem, BodySystem};
use frameworky::*;

#[derive(Debug, Default)]
struct TestSystem {}
impl SimpleSystem for TestSystem
{
    fn once(&mut self, context:&mut Context) {
        
    }
    fn update(&mut self, context:&mut Context) {
        let mut q = <(Entity, &mut Transform, &Body)>::query();
        for (e, t, b) in q.iter_mut(&mut context.world) {
            t.position.x += 0.01;
        }
    }
}

fn main()
{
    let mut f :Frameworky = Frameworky::default();
   // f.push_system(TestSystem::default());
    f.push_system(BodySystem::default());
    f.push_system(Kiss3DSystem::new("Sample!"));
    
    for _ in 0..10 {

        let x = random::<f32>() * 5.0;
        let z = random::<f32>() * 5.0;
        f.context.world.push(
            (Transform::new(x, 1.0, z), 
            Body::default()));
    }
    

    f.run();
}