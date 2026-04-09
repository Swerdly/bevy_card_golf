use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::prelude::*;

mod cards;
use cards::*;

const CARD_HEIGHT: f32  = 105.0;
const CARD_WIDTH: f32  = 75.0;

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
pub enum Owner{
    First,
    Second
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                // Explicitly request the Vulkan backend
                backends: Some(Backends::VULKAN),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GreetTimer(Timer::from_seconds(0.5, TimerMode::Once)))
        .add_plugins(MeshPickingPlugin, )
        .add_systems(Startup, setup)
        .add_systems(Update, queryTest)
        .run();
   
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
     asset_server: ResMut<AssetServer>,
){
    let mut deck = Deck{cards: Vec::new()};
    let card_mesh = meshes.add(Rectangle::new(CARD_WIDTH,CARD_HEIGHT));
    let card_material = materials.add(Color::srgb(0.2, 0.7, 0.9));

    commands.spawn(Camera2d);
    deck.initialize_deck();
    spawn_card(
        &mut commands, 
        &asset_server, 
        CardType::Joker, 
        Vec3::new(0.0, 0.0, 0.0),
        card_mesh,
        card_material
    );
    let card_mesh = meshes.add(Rectangle::new(CARD_WIDTH,CARD_HEIGHT));
    let card_material = materials.add(Color::srgb(0.2, 0.7, 0.9));
    spawn_card(
        &mut commands, 
        &asset_server, 
        CardType::Queen, 
        Vec3::new(0.0, 0.0, 0.0),
        card_mesh,
        card_material
    );
    let card_mesh = meshes.add(Rectangle::new(CARD_WIDTH,CARD_HEIGHT));
    let card_material = materials.add(Color::srgb(0.2, 0.7, 0.9));
    spawn_card(
        &mut commands, 
        &asset_server, 
        CardType::Joker, 
        Vec3::new(0.0, 0.0, 0.0),
        card_mesh,
        card_material
    );
}

fn queryTest(query: Query<&Card>, time: Res<Time>, mut timer: ResMut<GreetTimer>,){
    if timer.0.tick(time.delta()).just_finished() {
        for thing in query {
            if let CardType::Joker = thing.card_type{
                println!("joker here")
            }
        }
    }
}