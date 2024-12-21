use std::sync::RwLock;

use bevy::{app::{App, Plugin}, prelude::Resource};
use modules::{about::About, profiler::{Profiler, ProfilerModule}};

pub mod ui;
pub mod modules;

#[derive(Default, Resource)]
pub struct CorvidContext {
    modules: RwLock<Vec<Box<dyn modules::CorvidModule>>>,
}

unsafe impl Send for CorvidContext {}
unsafe impl Sync for CorvidContext {}

impl CorvidContext {
    pub fn new() -> Self {
        CorvidContext {
            modules: RwLock::new(Vec::new()),
        }
    }

    pub fn add_module(&self, module: Box<dyn modules::CorvidModule>) {
        self.modules.write().unwrap().push(module);
    }
}

pub struct CorvidPlugin {
}

impl CorvidPlugin {
    fn new() -> Self {
        CorvidPlugin {}
    }
}

impl Default for CorvidPlugin {
    fn default() -> Self {
        CorvidPlugin::new()
    }
}

impl Plugin for CorvidPlugin {
    fn build(&self, app: &mut App) {
        let context = CorvidContext::new();
        context.add_module(Box::new(About::default()));
        context.add_module(Box::new(ProfilerModule::default()));
        app.insert_resource(context);
    }
}