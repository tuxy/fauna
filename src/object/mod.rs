use macroquad::prelude::*;
use rapier3d::prelude::*;

pub enum ShapeKind {
    Plane(Vec2), // x, z
    Cuboid(Vec3), // x, y, z
    Sphere(f32), // radius
}

pub enum ObjectState {
    Dynamic,
    Static(Vec3), // Position data
}

pub struct Object {
    pub dynamic: ObjectState, // This won't be automatically detected
    pub shape_kind: ShapeKind,
    pub color: Color,
}

impl Object {
    // Checks what kind of shape used and draws shape using macroquad
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
    // Create an object with collision physics AND a rigidbody (Note that self isn't borrowed)
    pub fn make_dynamic(self, objects: &mut Vec<Object>, rigidbodies: &mut Vec<RigidBodyHandle>, position: Vec3, r_set: &mut RigidBodySet, c_set: &mut ColliderSet, raf: (f32, f32)) {
        let collider = self.check_collider(raf);
    
        let rigidbody = RigidBodyBuilder::dynamic()
        .translation(vector![position.x, position.y, position.z])
        .build();
    
        objects.push(self);
    
        let handle = r_set.insert(rigidbody);
        rigidbodies.push(handle);
        c_set.insert_with_parent(collider, handle, r_set);
    }

    pub fn make_static(self, objects: &mut Vec<Object>, c_set: &mut ColliderSet, raf: (f32, f32)) {
        let collider = self.check_collider(raf);
        c_set.insert(collider);
        objects.push(self);
    }

    // "raf" means restitution and friction
    fn check_collider(&self, raf: (f32, f32)) -> Collider {
        match self.shape_kind {
            ShapeKind::Sphere(r) => ColliderBuilder::ball(r)
                .restitution(raf.0) // TODO, add configurable bounciness?
                .friction(raf.1)
                .build(),
            ShapeKind::Cuboid(dim) => ColliderBuilder::cuboid(dim.x, dim.y, dim.z)
                .restitution(raf.0)
                .friction(raf.1)
                .build(),
            ShapeKind::Plane(dim) => ColliderBuilder::cuboid(dim.x, 0.1, dim.y)
                .restitution(raf.0)
                .friction(raf.1)
                .build(),
        }
    }
}