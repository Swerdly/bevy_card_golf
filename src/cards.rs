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
            Self::Ace => "sprites/ace.png",
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

// I just want a way to query the deck entity
#[derive(Component)]
pub struct DeckPile;


#[derive(Resource)]
pub struct Deck{
    pub cards: Vec<CardType>,
}

impl Deck{

    //Create card array at the beginning with every possible card
    pub fn initialize_deck(&mut self){
        self.cards.push(CardType::Ace);
        self.cards.push(CardType::Ace);
        self.cards.push(CardType::Ace);
        self.cards.push(CardType::Ace);
        self.cards.push(CardType::Two);
        self.cards.push(CardType::Two);
        self.cards.push(CardType::Two);
        self.cards.push(CardType::Two);
        self.cards.push(CardType::Three);
        self.cards.push(CardType::Three);
        self.cards.push(CardType::Three);
        self.cards.push(CardType::Three);
        self.cards.push(CardType::Four);
        self.cards.push(CardType::Four);
        self.cards.push(CardType::Four);
        self.cards.push(CardType::Four);
        self.cards.push(CardType::Five);
        self.cards.push(CardType::Five);
        self.cards.push(CardType::Five);
        self.cards.push(CardType::Five);
        self.cards.push(CardType::Six);
        self.cards.push(CardType::Six);
        self.cards.push(CardType::Six);
        self.cards.push(CardType::Six);
        self.cards.push(CardType::Seven);
        self.cards.push(CardType::Seven);
        self.cards.push(CardType::Seven);
        self.cards.push(CardType::Seven);
        self.cards.push(CardType::Eight);
        self.cards.push(CardType::Eight);
        self.cards.push(CardType::Eight);
        self.cards.push(CardType::Eight);
        self.cards.push(CardType::Nine);
        self.cards.push(CardType::Nine);
        self.cards.push(CardType::Nine);
        self.cards.push(CardType::Nine);
        self.cards.push(CardType::Ten);
        self.cards.push(CardType::Ten);
        self.cards.push(CardType::Ten);
        self.cards.push(CardType::Ten);
        self.cards.push(CardType::Jack);
        self.cards.push(CardType::Jack);
        self.cards.push(CardType::Jack);
        self.cards.push(CardType::Jack);
        self.cards.push(CardType::Queen);
        self.cards.push(CardType::Queen);
        self.cards.push(CardType::Queen);
        self.cards.push(CardType::Queen);
        self.cards.push(CardType::BlackKing);
        self.cards.push(CardType::BlackKing);
        self.cards.push(CardType::RedKing);
        self.cards.push(CardType::RedKing);
        self.cards.push(CardType::Joker);
        self.cards.push(CardType::Joker);
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

pub fn spawn_deck_entity(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let deck_texture = asset_server.load("sprites/cardBack_red5.png");

    commands.spawn((
        Sprite {
            image: deck_texture,
            custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(-300.0, 0.0, 0.0), // spawn pos for the deck, we can make this a param if we want to move it around
        DeckPile,
    )).observe(
        |trigger: On<Pointer<Click>>, 
         mut commands: Commands, 
         mut deck: ResMut<Deck>, 
         asset_server: Res<AssetServer>,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>| 
        {
            // pop from deck
            if let Some(card_to_spawn) = deck.cards.pop() {
                
                
                let spawn_position = Vec3::new(0.0, 0.0, 1.0);
                
                // 3. Spawn the actual card entity
                spawn_card(
                    &mut commands,
                    &asset_server,
                    card_to_spawn,
                    spawn_position,
                    meshes.add(Rectangle::default()),
                    materials.add(ColorMaterial::from(Color::WHITE)), 
                );
                
                println!("Spawned a card! {} cards left.", deck.cards.len());
            } else {
                println!("The deck is empty!");
            }
        }
    );
}

// add card related sytems and plugins here