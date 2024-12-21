use bevy::prelude::Commands;
use egui::Ui;

pub trait CorvidUi {
    fn title(&self) -> &str;
    fn draw(&self, commands: &Commands, ui: &mut Ui);
}