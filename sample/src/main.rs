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
    f.run();
}