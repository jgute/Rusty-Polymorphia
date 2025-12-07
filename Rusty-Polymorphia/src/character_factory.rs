use crate::adventurer::Adventurer;
use crate::creature::Creature;

pub struct CharacterFactory;

impl CharacterFactory {
    pub fn create_adventurer(name: &str) -> Adventurer {
        Adventurer::new(name)
    }

    pub fn create_creature(name: &str) -> Creature {
        Creature::new(name)
    }
}