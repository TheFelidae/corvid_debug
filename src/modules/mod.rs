use bevy::app::App;

use crate::ui::CorvidUi;

pub mod about;
pub mod profiler;

pub trait CorvidModule: CorvidUi {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn draw_settings(&self, ui: &mut egui::Ui);
    fn update(&mut self);
    fn build(&self, app: &mut App);
}