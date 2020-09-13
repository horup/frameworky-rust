use nalgebra::Point3;


#[derive(Clone, Copy, Debug)]
pub struct Transform
{
    pub position:Point3<f32>
}

impl Transform
{
    pub fn new(px:f32, py:f32, pz:f32) -> Transform
    {
        Transform {position:Point3::new(px, py, pz)}
    }
}