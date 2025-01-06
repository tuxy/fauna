use macroquad::prelude::*;
use rapier3d::prelude::*;
use object::*;
use ::rand::{thread_rng, Rng};
use ui::PhysicsOptions;

mod object;
mod physics;
mod ui;

#[macroquad::main("3D")]
async fn main() {

    // Initialise physics options with default values
    let mut ui_options = PhysicsOptions {
        gravity: -9.81,
        bounciness: 0.7,
        frction: 0.7,
        sim_speed: 1.0,
        reset: false // Starts off fresh
    };

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    let mut objects: Vec<Object> = vec![];
    let mut rigidbodies: Vec<RigidBodyHandle> = vec![];

    let mut rand = thread_rng();

    // Creates 10 spheres with a random position along with its rigidbody and collider
    for _ in 1..10 {
        let sphere = Object {
            dynamic: object::ObjectState::Dynamic, // Position is specified from make_dynamic function
            shape_kind: object::ShapeKind::Sphere(0.5),
            color: RED,
        };
        sphere.make_dynamic(
            &mut objects,
            &mut rigidbodies,
            Vec3::new(rand.gen_range(0.0..2.0), rand.gen_range(10.0..50.0), rand.gen_range(0.0..2.0)), 
            &mut rigid_body_set, 
            &mut collider_set
        );
    }

    let plane = Object {
        dynamic: object::ObjectState::Static(Vec3::new(0.0, 0.0, 0.0)), // 
        shape_kind: object::ShapeKind::Plane(Vec2::new(100.0, 100.0)),
        color: GRAY,
    };
    plane.make_static(&mut objects, &mut collider_set);

    // Initialise physics and values with default values
    let mut main_physics = physics::MainPhysicsStructure {
        gravity: vector![0.0, -9.81, 0.0],
        ..Default::default()
    };

    // Main loop
    loop {
        // Currently broken
        if ui_options.reset { // Checks if there is a pending reset for the simulation TODO
            main_physics = physics::MainPhysicsStructure {
                gravity: vector![0.0, ui_options.gravity, 0.0],
                ..Default::default()
            };
            ui_options.reset = !ui_options.reset;
        }

        // Starts the UI for configuring the simulation
        ui_options.init_ui();

        // Steps through every frame, with all of the collider and rigidbody sets, and ignores everything else
        main_physics.step(&mut rigid_body_set, &mut collider_set);

        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(-20., 10., 30.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        for (idx, object) in objects.iter().enumerate() {
            match object.dynamic {
                ObjectState::Dynamic => {
                    let body= &rigid_body_set[rigidbodies[idx]];
                    // Draws the object based on its parameters (self)
                    object.draw_object(body.translation()); 
                },
                ObjectState::Static(pos) => {
                    // Draws static object
                    object.draw_object(&vector![pos.x, pos.y, pos.z]);
                }
            }
        }

        // A grid, corresponding to the collision plane 
        set_default_camera();
        // Shows fps
        draw_text(format!("{}", get_fps()).as_str(), 50.0, 50.0, 15.0, BLACK);

        next_frame().await
    }
}