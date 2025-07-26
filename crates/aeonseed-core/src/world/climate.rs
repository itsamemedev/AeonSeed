use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum Season {
    #[default]
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum Weather {
    #[default]
    Clear,
    Rain,
    Storm,
    Snow,
}

/// Resource tracking current climate state affecting combat and crops.
#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct ClimateState {
    pub season: Season,
    pub weather: Weather,
}

impl Default for ClimateState {
    fn default() -> Self {
        Self { season: Season::Spring, weather: Weather::Clear }
    }
}

pub struct ClimatePlugin;

impl Plugin for ClimatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClimateState>();
        app.register_type::<ClimateState>();
        app.register_type::<Season>();
        app.register_type::<Weather>();
    }
}
