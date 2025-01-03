use macroquad::prelude::*;
use rapier3d::prelude::*;
use rayon::prelude::*;
use object::{Object, ShapeKind};
use ::rand::{thread_rng, Rng};

mod object;
mod physics;

#[macroquad::main("3D")]
async fn main() {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // PLANE -> Doesn't use struct for now
    let collider = ColliderBuilder::cuboid(10.0, 0.1, 10.0).build();
    collider_set.insert(collider);
    // PLANE

    let mut objects: Vec<Object> = vec![];
    let mut rigidbodies: Vec<RigidBodyHandle> = vec![];

    let mut rand = thread_rng();
    
    for _ in 1..100 {
        let sphere = Object {
            shape_kind: object::ShapeKind::Sphere(0.5),
        };

        let sphere_collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let sphere_rigidbody = RigidBodyBuilder::dynamic()
            .translation(vector![rand.gen_range(0.0..1.0), rand.gen_range(9.0..11.0), rand.gen_range(0.0..1.0)])
            .build();

        objects.push(sphere);

        let sphere_handle = rigid_body_set.insert(sphere_rigidbody);
        rigidbodies.push(sphere_handle);
        collider_set.insert_with_parent(sphere_collider, sphere_handle, &mut rigid_body_set);
    }

    // Initialise physics and values with default values
    let mut main_physics = physics::MainPhysicsStructure {
        ..Default::default()
    };
    // Main loop
    loop {

        main_physics.step(&mut rigid_body_set, &mut collider_set);

        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(-20., 10., 30.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        for (idx, object) in objects.iter().enumerate() {
            let body= &rigid_body_set[rigidbodies[idx]];
            match object.shape_kind {
                ShapeKind::Sphere(r) => draw_sphere(
                    Vec3::new(body.translation().x, body.translation().y, body.translation().z),
                    r,
                    None,
                    RED
                ),
                ShapeKind::Cuboid(dim) => draw_cube(
                    Vec3::new(body.translation().x, body.translation().y, body.translation().z),
                    dim,
                    None,
                    GRAY
                ),
                ShapeKind::Plane(dim) => draw_plane(Vec3::new(body.translation().x, body.translation().y, body.translation().z), 
                    dim, 
                    None, 
                    BLACK
                ),
            };
        }

        draw_grid(20, 1., BLACK, GRAY);

        set_default_camera();
        draw_text(format!("{}", get_fps()).as_str(), 50.0, 50.0, 15.0, BLACK);

        next_frame().await
    }
}