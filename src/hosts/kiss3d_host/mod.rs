#[allow(dead_code)]
mod arc_ball_modified;
use arc_ball_modified::ArcBall;

use std::{collections::HashMap};
use legion::*;
use kiss3d::{window::Window, ncollide3d::math::Translation, scene::SceneNode, event::WindowEvent, event::Action, event::MouseButton, nalgebra::Point3, nalgebra::UnitQuaternion, nalgebra::Vector3, window::State, camera::Camera, planar_camera::PlanarCamera, renderer::Renderer, post_processing::PostProcessingEffect};

use crate::{Frameworky, SimpleSystem, events::KeyEvent, events::MouseEvent, events::MouseEventType, systems::Kiss3DSystem};
use crate::components::*;

//use super::arc_ball_modified::ArcBallModified;
pub struct Kiss3DHost
{
    arc_ball_camera:ArcBall,
    nodes:HashMap<Entity, SceneNode>,
    frameworky:Frameworky,
    prev_state:World
}

impl Kiss3DHost
{
    pub fn start(mut frameworky:Frameworky, title:&str)
    {
        frameworky.push_system(Kiss3DSystem::default());
        let arc_ball = ArcBall::new(
            Point3::new(0.0, 20.0, 20.0),
            Point3::origin());

        let window = Window::new(title);
        let host = Kiss3DHost {
            arc_ball_camera: arc_ball,
            nodes:HashMap::new(),
            frameworky,
            prev_state:World::default()
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
        let cam:&mut dyn Camera = &mut self.arc_ball_camera;
        (Some(cam), None, None, None)
    }

}
