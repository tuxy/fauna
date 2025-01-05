use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets,
};

pub struct PhysicsOptions {
    pub gravity: f32,
    pub bounciness: f32,
    pub frction: f32, // TODO, how to put into the objects themselves.
    pub sim_speed: f32,
    pub reset: bool,
}

impl PhysicsOptions {
    pub fn init_ui(&mut self) {
        widgets::Window::new(hash!(), vec2(0., 0.), vec2(500., 200.))
            .label("Options")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                // Sliders for gravity, bounciness, friction and simulation speed
                ui.slider(hash!(), "Gravity     [-10 .. 10]", -10f32..10f32, &mut self.gravity);
                ui.slider(hash!(), "Bounciness  [0.0 .. 1.0]", 0f32..1f32, &mut self.bounciness);
                ui.slider(hash!(), "Friction    [0.0 .. 1.0]", 0f32..1f32, &mut self.frction);
                ui.slider(hash!(), "Sim Speed   [0.0 .. 1.0]", 0f32..1f32, &mut self.sim_speed);
                if ui.button(Vec2::new(0., 120.), "Apply & reset") {
                    self.reset = true;
                    println!("Simulation Reset");
                }
            });
    }
}