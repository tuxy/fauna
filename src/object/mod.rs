use macroquad::prelude::*;

pub enum ShapeKind {
    Plane(Vec2), // x, z
    Cuboid(Vec3), // x, y, z
    Sphere(f32), // radius
}

pub struct Object {
    pub shape_kind: ShapeKind,
}