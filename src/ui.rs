use egui::Ui;

pub trait CorvidUi {
    fn title(&self) -> &str;
    fn draw(&self, ui: &mut Ui);
}