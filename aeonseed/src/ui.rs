use bevy::prelude::*;
use crate::seed::SeedInfo;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Options,
    Character,
    InGame,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
           .add_systems(OnEnter(GameState::Menu), setup_menu)
           .add_systems(OnExit(GameState::Menu), cleanup_menu)
           .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
           .add_systems(OnEnter(GameState::InGame), setup_hud)
           .add_systems(Update, update_fps.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
struct MenuUi;

#[derive(Component)]
struct FpsText;

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            MenuUi,
        ))
        .with_children(|parent| {
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            parent.spawn(TextBundle::from_section(
                "AeonSeed",
                TextStyle { font: font.clone(), font_size: 40.0, color: Color::WHITE },
            ));
            for label in ["Start", "Optionen", "Charakter"] {
                parent
                    .spawn(ButtonBundle::default())
                    .with_children(|p| {
                        p.spawn(TextBundle::from_section(label, TextStyle { font: font.clone(), font_size: 24.0, color: Color::BLACK }));
                    });
            }
        });
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUi>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

fn menu_input(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut texts: Query<&mut Text>,
) {
    for (interaction, children) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(text_entity) = children.iter().next() {
                if let Ok(mut text) = texts.get_mut(*text_entity) {
                    match text.sections[0].value.as_str() {
                        "Start" => next_state.set(GameState::InGame),
                        "Optionen" => next_state.set(GameState::Options),
                        "Charakter" => next_state.set(GameState::Character),
                        _ => {},
                    }
                }
            }
        }
    }
}

fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>, seed: Res<SeedInfo>) {
    commands.spawn((TextBundle::from_sections([
        TextSection::new("Seed: ", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 20.0, color: Color::WHITE }),
        TextSection::new(seed.id.clone(), TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 20.0, color: Color::WHITE }),
        TextSection::new("  FPS: 0", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 20.0, color: Color::WHITE }),
    ]), FpsText));
}

fn update_fps(diagnostics: Res<bevy::diagnostic::DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    if let Some(fps) = diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.average()) {
        for mut text in &mut query {
            if let Some(section) = text.sections.get_mut(2) {
                section.value = format!("  FPS: {:.0}", fps);
            }
        }
    }
}
