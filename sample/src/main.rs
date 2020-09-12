use crate::components::Body;
use crate::components::Transform;
use systems::{SDLSystem, Kiss3DSystem};
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
    f.push_system(Kiss3DSystem::new("Sample!"));
    
    for _ in 0..10 {

        let x = random::<f32>() * 5.0;
        let z = random::<f32>() * 5.0;
        f.context.world.push(
            (Transform::new(x, 0.0, z), 
            Body::default()));
    }
    

    f.run();
}