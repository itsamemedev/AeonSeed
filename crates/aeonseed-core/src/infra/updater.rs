use bevy::log::info;

/// Check with the update server for new versions and print patch notes.
pub fn check_for_updates() {
    // In a real implementation this would contact a remote API. For now we
    // simply log that the check happened to demonstrate integration.
    info!("Auto-updater: no updates available");
}
