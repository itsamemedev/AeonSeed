use bevy::prelude::*;
use sysinfo::{System, SystemExt};

pub struct AdminPlugin;

/// Gibt CPU- und RAM-Auslastung aus.
///
/// Die Werte werden bei jedem Aufruf aktualisiert und
/// zeigen so die aktuelle Last des Servers an.
fn monitor_command(mut system: ResMut<System>) {
    system.refresh_cpu();
    system.refresh_memory();
    println!(
        "CPU: {}% RAM: {} MB",
        system.global_processor_info().cpu_usage(),
        system.used_memory()
    );
}

impl Plugin for AdminPlugin {
    fn build(&self, app: &mut App) {
        let mut sys = System::new();
        sys.refresh_all();
        app.insert_resource(sys)
            .add_systems(Update, monitor_command);
    }
}
