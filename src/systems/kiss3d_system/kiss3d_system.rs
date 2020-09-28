use std::{any::Any, collections::HashMap, f32::consts::PI};
use kiss3d::{event::Action, event::MouseButton, event::WindowEvent, scene::SceneNode, window::Window};
use legion::{Entity, world::Duplicate};
use nalgebra::{UnitQuaternion, Vector3};
use nphysics3d::math::Translation;
use legion::query::*;

use crate::{Context, SimpleSystem, components::Body, components::Shape, components::Transform, events::KeyEvent, events::MouseEvent, events::MouseEventType};

#[derive(Default)]
pub struct Kiss3DSystem
{
    pub window:Option<*mut Window>,
    nodes:HashMap<Entity, SceneNode>,
}

impl SimpleSystem for Kiss3DSystem
{
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(self)
    }

    fn update_fixed(&mut self, context:&mut Context) {
        if let Some(window_ptr) = self.window
        {
            // window is a pointer to the underlying owner, which is unsafe
            unsafe {
                let window = &mut (*window_ptr);
                self.update_fixed(context, window);
            }
        }
    }

    fn update(&mut self, context:&mut Context) {
        if let Some(window_ptr) = self.window
        {
             // window is a pointer to the underlying owner, which is unsafe
            unsafe {
                let window = &mut (*window_ptr);
                self.update(context, window);
            }
        }
    }
}

impl Kiss3DSystem
{
    fn process_events(&mut self, context:&mut Context, window:&mut Window) {
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

                    context.push_event(e);
                },
                WindowEvent::Key(key, action, _modifier) => 
                {
                    let e = KeyEvent { 
                        down:if action == Action::Press { true } else { false },
                        key:key as u32
                    };

                    context.push_event(e);
                }
                _ => {}
            }
        }
    }

    
    fn sync_from(&mut self,  context:&mut Context, window:&mut Window) {
        let bodies = &mut self.nodes;
        let world = &mut context.world;
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

    
    pub fn update_fixed(&mut self, context:&mut Context, window:&mut Window)
    {
        let world = &mut context.world;
        //self.prev_state.clear();
        let mut duplicater = Duplicate::default();
        duplicater.register_copy::<Transform>();
        //self.prev_state.clone_from(world, &legion::any(), &mut duplicater);
    }

    pub fn update(&mut self, context:&mut Context, window:&mut Window)
    {
        self.process_events(context, window);
        self.sync_from(context, window);
    }
    


}




