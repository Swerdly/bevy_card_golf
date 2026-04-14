use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::prelude::*;

use rand::seq::SliceRandom; // Required trait for .shuffle()
use rand::Rng;

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
        .insert_resource(Deck{cards: Vec::new()})
        .add_plugins(MeshPickingPlugin, )
        .add_systems(Startup, (setup, set_deck_system).chain())
        .run();
   
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
){
    commands.spawn(Camera2d);

    let card_mesh = meshes.add(Rectangle::new(CARD_WIDTH,CARD_HEIGHT));
    let card_material = materials.add(Color::srgb(0.2, 0.7, 0.9));

    // for card_type in &deck.cards{
    //     spawn_card(
    //         &mut commands, 
    //         &asset_server,
    //         *card_type, // dereferencing card type so it can be read and not moved from vector
    //         Vec3::new(0.0, 0.0, 0.0),
    //         card_mesh.clone(), // cloning so you don't repeatedly have to create a new card_mesh and card_material in the loop
    //         card_material.clone()
    //     );
    // }

    spawn_deck_entity(&mut commands, &asset_server, card_mesh.clone());
}

pub fn set_deck_system(mut deck: ResMut<Deck>){
    deck.create_deck();   
    let mut rng = rand::rng();
    deck.cards.shuffle(&mut rng);
}

// fn queryTest(query: Query<&Card>, time: Res<Time>, mut timer: ResMut<GreetTimer>,){
//     if timer.0.tick(time.delta()).just_finished() {
//         for thing in query {
//             if let CardType::Joker = thing.card_type{
//                 println!("joker here")
//             }
//         }
//     }
// }