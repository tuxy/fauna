use macroquad::prelude::*;
use rapier3d::prelude::*;
use object::Object;
use ::rand::{thread_rng, Rng};
use ui::PhysicsOptions;

mod object;
mod physics;
mod ui;

#[macroquad::main("3D")]
async fn main() {

    // Initialise physics options with default values
    let mut physics_options = PhysicsOptions {
        gravity: -9.81,
        bounciness: 0.7,
        frction: 0.7,
        sim_speed: 1.0,
        reset: false // Starts off fresh
    };

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // PLANE -> Doesn't use struct for now
    let collider = ColliderBuilder::cuboid(10.0, 0.1, 10.0).build();
    collider_set.insert(collider);
    // PLANE

    let mut objects: Vec<Object> = vec![];
    let mut rigidbodies: Vec<RigidBodyHandle> = vec![];

    let mut rand = thread_rng();

    // Creates 10 spheres with a random position along with its rigidbody and collider
    for _ in 1..10 {
        let sphere = Object {
            shape_kind: object::ShapeKind::Sphere(0.5),
            color: RED,
        };
        object::draw_stateful(
            sphere,
            &mut objects,
            &mut rigidbodies,
            Vec3::new(rand.gen_range(0.0..2.0), rand.gen_range(10.0..50.0), rand.gen_range(0.0..2.0)), 
            &mut rigid_body_set, 
            &mut collider_set
        );
    }

    // Initialise physics and values with default values
    let mut main_physics = physics::MainPhysicsStructure {
        gravity: vector![0.0, -9.81, 0.0],
        ..Default::default()
    };

    // Main loop
    loop {
        // Currently broken
        if physics_options.reset { // Checks if there is a pending reset for the simulation TODO
            main_physics = physics::MainPhysicsStructure {
                gravity: vector![0.0, physics_options.gravity, 0.0],
                ..Default::default()
            };
            physics_options.reset = !physics_options.reset;
        }

        physics_options.init_ui();

        // Steps through every frame
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

            let pos = body.translation();

            // Draws the object based on its parameters
            object.draw_object(pos);    
        }

        // A grid, corresponding to the collision plane 
        draw_grid(20, 1., BLACK, GRAY);

        set_default_camera();
        // Shows fps
        draw_text(format!("{}", get_fps()).as_str(), 50.0, 50.0, 15.0, BLACK);

        next_frame().await
    }
}