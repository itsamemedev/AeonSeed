use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CurrencyExchange {
    pub aeonium_rate: f32,
}

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrencyExchange>();
    }
}
