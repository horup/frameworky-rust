use nalgebra::Vector3;

use super::Precision;

#[derive(Clone, Copy, Debug)]
pub struct Transform
{
    pub position:Vector3<Precision>,
    pub direction:Vector3<Precision>
}

impl Transform
{
    pub fn new(px:Precision, py:Precision, pz:Precision) -> Transform
    {
        Transform {
            position:Vector3::new(px, py, pz),
            direction:Vector3::new(1.0, 0.0, 0.0)
        }
    }

    pub fn lerp(&self, prev:&Transform, alpha:Precision) -> Transform
    {
        let current_p = self.position;
        let prev_p = prev.position;
        let v:Vector3<Precision> = current_p - prev_p;
        let p:Vector3<Precision> = prev_p + v.scale(alpha as f32);

        Transform {
            position:p,
            direction:self.direction
        }
    }
}