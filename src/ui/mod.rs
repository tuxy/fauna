use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets,
};

pub struct UiOptions {
    pub item_count: f32,
    pub gravity: f32,
    pub restitution: f32,
    pub frction: f32, // TODO, how to put into the objects themselves.
    pub sim_speed: f32,
    pub reset: bool,
}

impl UiOptions {
    pub fn init_ui(&mut self) {
        widgets::Window::new(hash!(), vec2(0., 0.), vec2(500., 200.))
            .label("Options")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                // Sliders for gravity, bounciness, friction and simulation speed
                ui.slider(hash!(), "Item Count  [1 .. 600]", 1f32..600f32, &mut self.item_count);
                ui.slider(hash!(), "Gravity     [-20 .. 20]", -20f32..20f32, &mut self.gravity);
                ui.slider(hash!(), "Bounciness  [0.0 .. 1.0]", 0f32..1f32, &mut self.restitution);
                ui.slider(hash!(), "Friction    [0.0 .. 1.0]", 0f32..1f32, &mut self.frction);
                ui.slider(hash!(), "Sim Speed   [0.0 .. 1.0]", 0f32..1f32, &mut self.sim_speed);
                if ui.button(Vec2::new(0., 120.), "Apply & reset") {
                    self.reset = true;
                    println!("Simulation Reset");
                }
            });
    }
}