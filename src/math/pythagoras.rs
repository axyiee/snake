use bevy::math::Vec3;

pub fn pythagoras(from: Vec3, to: Vec3) -> f32 {
    let x = to.x - from.x;
    let y = to.y - from.y;
    ((x * x) + (y * y)).sqrt()
}
