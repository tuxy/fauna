use macroquad::prelude::*;
use rapier3d::prelude::*;

pub enum ShapeKind {
    Plane(Vec2), // x, z
    Cuboid(Vec3), // x, y, z
    Sphere(f32), // radius
}

pub struct Object {
    pub shape_kind: ShapeKind,
    pub color: Color,
}

impl Object {
    pub fn draw_object(&self, pos: &nalgebra::Matrix<f32, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 3, 1>>) {
        match self.shape_kind {
            ShapeKind::Sphere(r) => draw_sphere(
                Vec3::new(pos.x, pos.y, pos.z),
                r,
                None,
                self.color
            ),
            ShapeKind::Cuboid(dim) => draw_cube(
                Vec3::new(pos.x, pos.y, pos.z),
                dim,
                None,
                self.color
            ),
            ShapeKind::Plane(dim) => draw_plane(Vec3::new(pos.x, pos.y, pos.z), 
                dim, 
                None, 
                self.color
            ),
        }
    }
}

pub fn draw_stateful(object: Object, objects: &mut Vec<Object>, rigidbodies: &mut Vec<RigidBodyHandle>, position: Vec3, r_set: &mut RigidBodySet, c_set: &mut ColliderSet) {
    let collider = match object.shape_kind {
        ShapeKind::Sphere(r) => ColliderBuilder::ball(r)
            .restitution(0.7) // TODO, add configurable bounciness?
            .friction(0.7)
            .build(),
        ShapeKind::Cuboid(dim) => ColliderBuilder::cuboid(dim.x, dim.y, dim.z)
            .restitution(0.7)
            .friction(0.7)
            .build(),
        ShapeKind::Plane(dim) => ColliderBuilder::cuboid(dim.x, dim.y, 0.1)
            .restitution(0.7)
            .friction(0.7)
            .build(),
    };

    let rigidbody = RigidBodyBuilder::dynamic()
    .translation(vector![position.x, position.y, position.z])
    .build();

    objects.push(object);

    let handle = r_set.insert(rigidbody);
    rigidbodies.push(handle);
    c_set.insert_with_parent(collider, handle, r_set);
}