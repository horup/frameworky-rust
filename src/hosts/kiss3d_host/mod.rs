#[allow(dead_code)]
mod arc_ball_modified;
use arc_ball_modified::ArcBall;
use world::{Duplicate, Merger};

use std::{cell::RefCell, collections::HashMap, f32::consts::PI, rc::Rc, rc::Weak};
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

    fn sync_from(&mut self,  window:&mut Window) {
        let bodies = &mut self.nodes;
        let world = &mut self.frameworky.context.world;
        let col = || rand::random::<f32>();
        let test = Transform::new(0.0, 0.0, 0.0);
        test.position.x;

        <(Entity, &mut Transform, &mut Body)>::query().for_each_mut(world, |(k, t, b)| {
            if !bodies.contains_key(k) {
                if b.shape == Shape::Sphere {
                    let mut sphere = window.add_sphere(0.5);
                    sphere.set_color(col(), col(), col());
                    bodies.insert(*k, sphere);
/*
                    let mut quad = window.add_quad(1.0, 1.0, 1, 1);
                    quad.set_color(col(), col(), col());
                    bodies.insert(*k, quad);*/
                }
                else if b.shape == Shape::Plane {
                    let size = 100.0;
                    let mut plane = window.add_quad(size, size, 1, 1);
                    let rot = UnitQuaternion::from_axis_angle(&Vector3::<f32>::x_axis(), PI / 2.0);
                    plane.append_rotation(&rot);
                    bodies.insert(*k, plane);
                }
            }

            if let Some(node) = bodies.get_mut(k) {
                let p = &t.position;
                node.set_local_translation(Translation::new(p.x, p.y, p.z));
            }
           
        });
    }

    fn process_events(&mut self, window:&mut Window) {
        for e in window.events().iter() {
            match e.value 
            {
                WindowEvent::MouseButton(mb, a, _m) =>
                {
                    let pos = window.cursor_pos().unwrap();
                    let b = if mb == MouseButton::Button1 { 0 } else { 1 };

                    let e = MouseEvent
                    {
                        button:b,
                        event_type:if a == Action::Press { MouseEventType::ButtonDown } else { MouseEventType::ButtonUp },
                        screen_x:pos.0 as i32,
                        screen_y:pos.1 as i32
                    };

                    self.frameworky.context.push_event(e);
                },
                WindowEvent::Key(key, action, _modifier) => 
                {
                    let e = KeyEvent { 
                        down:if action == Action::Press { true } else { false },
                        key:key as u32
                    };

                    self.frameworky.context.push_event(e);
                }
                _ => {}
            }
        }
    }

    pub fn fixed_update(&mut self, _window:&mut Window)
    {
        let world = &mut self.frameworky.context.world;
        self.prev_state.clear();
        let mut duplicater = Duplicate::default();
        duplicater.register_copy::<Transform>();
        self.prev_state.clone_from(world, &legion::any(), &mut duplicater);
    }
}

struct Kiss3DData
{
    pub window:Weak<RefCell<Window>>
}

impl State for Kiss3DHost
{
    fn step<'a>(&mut self, window: &mut Window) {

        self.process_events(window);

        let kiss3d_system:Option<&mut Kiss3DSystem> = self.frameworky.get_system_mut();
        if let Some(s) = kiss3d_system
        {
            // pointers are used since Window is owner
            // thus an unsafe unsafe circular reference
            let p:*mut Window = window;
            s.window = Some(p);
        }

        self.frameworky.update();
        self.sync_from(window);
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
