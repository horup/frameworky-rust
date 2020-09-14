use nalgebra::Vector3;

type Precision = f32;
#[derive(Clone, Copy, Debug)]
pub struct Transform
{
    pub position:Vector3<Precision>
}

impl Transform
{
    pub fn new(px:Precision, py:Precision, pz:Precision) -> Transform
    {
        Transform {position:Vector3::new(px, py, pz)}
    }
}