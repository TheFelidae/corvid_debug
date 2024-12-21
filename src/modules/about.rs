use bevy::prelude::Commands;

use crate::ui::CorvidUi;

use super::CorvidModule;

#[derive(Default)]
pub struct About;

impl CorvidModule for About {
    fn id(&self) -> &str {
        "corvid.about"
    }

    fn description(&self) -> &str {
        "About Corvid Debug"
    }

    fn draw_settings(&self, _ui: &mut egui::Ui) {}

    fn update(&mut self) {}
    
    fn build(&self, app: &mut bevy::prelude::App) {
    }
}

impl CorvidUi for About {
    fn title(&self) -> &str {
        "About"
    }

    fn draw(&self, commands: &Commands, ui: &mut egui::Ui) {
        // TODO: Add content
        ui.label("TODO");
    }
}