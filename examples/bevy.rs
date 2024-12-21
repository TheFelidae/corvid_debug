use bevy::{app::{App, Update}, prelude::ResMut};
use corvid_debug::{CorvidPlugin, modules::profiler::Profiler, CorvidContext};

fn example_system(
    mut profiler: ResMut<Profiler>,
) {
    let _record = 
    println!("Hello, world!");
}


fn main() {
    let mut app = App::new();
    app.add_plugins(CorvidPlugin::default());
    app.add_systems(Update, example_system);

    app.run();
}