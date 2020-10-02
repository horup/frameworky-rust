#[allow(dead_code)]
mod arc_ball_modified;
use arc_ball_modified::ArcBall;
use nalgebra::Point2;
use nphysics3d::math::Point;

use kiss3d::{window::Window, nalgebra::Point3, 
    window::State, camera::Camera, planar_camera::PlanarCamera, renderer::Renderer, post_processing::PostProcessingEffect};

use crate::{Frameworky, systems::Kiss3DSystem};

//use super::arc_ball_modified::ArcBallModified;
pub struct Kiss3DHost
{
    frameworky:Frameworky
}

impl Kiss3DHost
{
    pub fn start(mut frameworky:Frameworky, title:&str)
    {
        frameworky.push_system(Kiss3DSystem::default());
        let window = Window::new(title);
        let host = Kiss3DHost {
            frameworky
        };

        window.render_loop(host);
    }

}

impl State for Kiss3DHost
{
    fn step<'a>(&mut self, window: &mut Window) {
        let kiss3d_system:Option<&mut Kiss3DSystem> = self.frameworky.get_system_mut();
        if let Some(s) = kiss3d_system
        {
            // pointers are used since Window is owner
            // thus an unsafe circular reference
            let p:*mut Window = window;
            s.window = Some(p);
        }

        self.frameworky.update();
    }

    fn cameras_and_effect_and_renderer(&mut self) -> (
        Option<&mut dyn Camera>,
        Option<&mut dyn PlanarCamera>,
        Option<&mut dyn Renderer>,
        Option<&mut dyn PostProcessingEffect>) 
    {
        if let Some(system) = self.frameworky.get_system_mut::<Kiss3DSystem>() {
            let cam:&mut dyn Camera = &mut system.camera;
            return (Some(cam), None, None, None);
        }

        return (None, None, None, None);
    }

}
