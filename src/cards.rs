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

#[derive(Component, Clone, Copy)]
pub struct Card{
    value: i32,
    ability: CardAbility,
    pub card_type: CardType,
}

pub struct Deck{
    pub cards: Vec<Card>,
}

impl Deck{

    //Create card array at the beginning with every possible card
    pub fn initialize_deck(&mut self){
        let mut cards_in_deck = 0;

        while cards_in_deck < 54 {
            match cards_in_deck {
                0..=3 => {
                    let card = Card{ value: 1, ability: CardAbility::None, card_type: CardType::Ace };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                4..=7 => {
                    let card = Card{ value: 2, ability: CardAbility::None, card_type: CardType::Two };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                8..=11 => {
                    let card = Card{ value: 3, ability: CardAbility::None, card_type: CardType::Three };
                    self.cards.push(card);
                    cards_in_deck += 1;
                }
                12..=15 => {
                    let card = Card{ value: 4, ability: CardAbility::None, card_type: CardType::Four };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                16..=19 => {
                    let card = Card{ value: 5, ability: CardAbility::None, card_type: CardType::Five };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                20..=23 => {
                    let card = Card{ value: 6, ability: CardAbility::None, card_type: CardType::Six };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                24..=27 => {
                    let card = Card{ value: 7, ability: CardAbility::SelfPeek, card_type: CardType::Seven };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                28..=31 => {
                    let card = Card{ value: 8, ability: CardAbility::SelfPeek, card_type: CardType::Eight };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                32..=35 => {
                    let card = Card{ value: 9, ability: CardAbility::ElsePeek, card_type: CardType::Nine };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                36..=39 => {
                    let card = Card{ value: 10, ability: CardAbility::ElsePeek, card_type: CardType::Ten };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                40..=43 => {
                    let card = Card{ value: 11, ability: CardAbility::BlindSwap, card_type: CardType::Jack };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                44..=47 => {
                    let card = Card{ value: 12, ability: CardAbility::BlindSwap, card_type: CardType::Queen };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                48..=49 => {
                    let card = Card{ value: 13, ability: CardAbility::PeekAndSwap, card_type: CardType::BlackKing };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                50..=51 => {
                    let card = Card{ value: 0, ability: CardAbility::None, card_type: CardType::RedKing };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                52..=53 => {
                    let card = Card{ value: -1, ability: CardAbility::None, card_type: CardType::Joker };
                    self.cards.push(card);
                    cards_in_deck += 1;
                },
                _ => {
                    println!("silence rust");
                },
            }
        }
        let mut card_num = 1;
        for item in &self.cards{
            println!("{card_num}. {}", item.value);
            card_num += 1;
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

    let card = Card{
        value: value,
        ability: ability,
        card_type: card_type,
    };
    
    commands.spawn((
        Sprite {
            image: texture,
            custom_size:  Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
            ..default()
        },
        Transform::from_translation(translation),
        card,
        // card_type,                  
        // CardValue { value },        
        // ability,    
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