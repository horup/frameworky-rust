use std::{any::Any, collections::HashMap, f32::consts::PI};
use kiss3d::{event::Action, event::MouseButton, event::WindowEvent, scene::SceneNode, window::Window};
use legion::{Entity, World, world::Duplicate};
use nalgebra::{UnitQuaternion, Vector3};
use nphysics3d::math::Translation;
use legion::query::*;

use crate::{Context, SimpleSystem, components::Body, components::Shape, components::Transform, events::KeyEvent, events::MouseEvent, events::MouseEventType};

#[derive(Default)]
pub struct Kiss3DSystem
{
    pub window:Option<*mut Window>,
    nodes:HashMap<Entity, SceneNode>,
    prev_state:World
}

impl SimpleSystem for Kiss3DSystem
{
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(self)
    }

    fn before_fixed_update(&mut self, context:&mut Context) {
        if let Some(window_ptr) = self.window
        {
            // window is a pointer to the underlying owner, which is unsafe
            unsafe {
                let window = &mut (*window_ptr);
                self.before_fixed_update(context, window);
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

    fn render(&mut self, context:&mut Context, window:&mut Window)
    {
        let nodes = &mut self.nodes;
        let prev_state = &self.prev_state;
        let world = &mut context.world;
        let alpha = context.time.alpha;
        <(Entity, &Transform, &Body)>::query().for_each(prev_state, |(e, t, b)| {

            if let Some(current) = world.entry(*e)
            {
                if let Ok(current_t) = current.get_component::<Transform>()
                {
                    if let Some(node) = nodes.get_mut(e) {
                        let prev_t = &t;
                        let current_p = current_t.position;
                        let prev_p = prev_t.position;
                        let mut v:Vector3<f32> = current_p - prev_p;
                        v.normalize();
                        let p = prev_p + v.scale(alpha as f32);

                        node.set_local_translation(Translation::new(p.x, p.y, p.z));
                    }
                }

            }
            else
            {
                // TODO remove node
            }
          
        });
    }

    
    pub fn before_fixed_update(&mut self, context:&mut Context, window:&mut Window)
    {
        let world = &mut context.world;
        self.prev_state.clear();
        <(Entity, &Transform, &Body)>::query().for_each(world, |(e, t, b)| {
            self.prev_state.push_with_id(*e, (t.clone(), b.clone()));
            let nodes = &mut self.nodes;

            if !nodes.contains_key(e)
            {
                let col = || rand::random::<f32>();
                if b.shape == Shape::Sphere {
                    let mut sphere = window.add_sphere(0.5);
                    sphere.set_color(col(), col(), col());
                    nodes.insert(*e, sphere);
                }
                else if b.shape == Shape::Plane {
                    let size = 100.0;
                    let mut plane = window.add_quad(size, size, 1, 1);
                    let rot = UnitQuaternion::from_axis_angle(&Vector3::<f32>::x_axis(), PI / 2.0);
                    plane.append_rotation(&rot);
                    nodes.insert(*e, plane);
                }
            }

        });
    }

    pub fn update(&mut self, context:&mut Context, window:&mut Window)
    {
        self.process_events(context, window);
        //self.sync_from(context, window);
        self.render(context, window);
    }

}




