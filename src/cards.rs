use bevy::prelude::*;

const CARD_HEIGHT: f32  = 105.0;
const CARD_WIDTH: f32  = 75.0;

#[derive(Component, Clone, Copy)]
pub struct CardValue {
    pub value: i32,
}




#[derive(Component, Clone, Copy)]
pub enum CardAbility {
    PeekAndSwap,
    BlindSwap,
    SelfPeek,
    ElsePeek,
    None

}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum CardType{
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    RedKing,
    BlackKing,
    Ace,
    Joker

}


// these functions let you do CardType::Ace.funcName() to get other properties of the card
impl CardType {
    pub fn value(&self) -> i32 {
        
        match self{
            Self::Ace => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Jack => 11,
            Self::Queen => 12,
            Self::RedKing => 0,
            Self::BlackKing => 13,       
            Self::Joker => -1,
        }
    }

    pub fn ability(&self) -> CardAbility {
        match self{
            Self::Seven => CardAbility::SelfPeek,
            Self::Eight => CardAbility::SelfPeek,
            Self::Nine => CardAbility::ElsePeek,
            Self::Ten => CardAbility::ElsePeek,
            Self::Jack => CardAbility::BlindSwap,
            Self::Queen => CardAbility::BlindSwap,
            Self::RedKing => CardAbility::PeekAndSwap,
            Self::BlackKing => CardAbility::PeekAndSwap,
            _ => CardAbility::None,
        }
    }   

    pub fn sprite(&self) -> &'static str{
         match self{
            Self::Ace => "sprites/1.png",
            Self::Two => "sprites/2.png",
            Self::Three => "sprites/3.png",
            Self::Four => "sprites/4.png",
            Self::Five => "sprites/5.png",
            Self::Six => "sprites/6.png",
            Self::Seven => "sprites/7.png",
            Self::Eight => "sprites/8.png",
            Self::Nine => "sprites/9.png",
            Self::Ten => "sprites/10.png",
            Self::Jack => "sprites/jack.png",
            Self::Queen => "sprites/queen.png",
            Self::RedKing => "sprites/redKing.png",
            Self::BlackKing => "sprites/blackKing.png",       
            Self::Joker => "sprites/joker.png",
        }
    }

}

pub fn spawn_card(
    commands: &mut Commands,
    asset_server: &AssetServer,
    card_type: CardType, // type of card that will be spawned
    translation: Vec3, // where are we spawning it
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) -> Entity {
    
    // get associated values through card type functions
    let value = card_type.value();
    let ability = card_type.ability();
    let texture =  asset_server.load(  card_type.sprite());

    
    commands.spawn((
        Sprite {
            image: texture,
            custom_size:  Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
            ..default()
        },
        Transform::from_translation(translation),
        card_type,                  
        CardValue { value },        
        ability,    
        children![(
    
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_xyz(0.0, 0.0, 0.0),
        )]                
    )).observe(
            |trigger: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>| {
                
                if let Ok(mut transform) = transforms.get_mut(trigger.entity.entity()) {
                    
                    
                    transform.translation.x += trigger.event().delta.x;
                    transform.translation.y -= trigger.event().delta.y;
                }
            }
        ).id()
}

// add card related sytems and plugins here