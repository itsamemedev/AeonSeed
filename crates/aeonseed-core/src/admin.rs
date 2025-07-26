use bevy::prelude::*;
use sysinfo::{System, SystemExt};

pub struct AdminPlugin;

fn monitor_command(system: Res<System>) {
    println!("CPU: {}% RAM: {} MB", system.global_processor_info().cpu_usage(), system.used_memory());
}

impl Plugin for AdminPlugin {
    fn build(&self, app: &mut App) {
        let mut sys = System::new();
        sys.refresh_all();
        app.insert_resource(sys)
            .add_systems(Update, monitor_command);
    }
}
