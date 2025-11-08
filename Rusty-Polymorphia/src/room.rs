use rand::Rng;

use crate::adventurer::Adventurer;
use crate::creature::Creature;

#[derive(Clone)]
pub struct Room {
    pub adventurer: Option<Adventurer>,
    pub creature: Option<Creature>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            adventurer: None,
            creature: None,
        }
    }

    pub fn get_adventurer(&self) -> Option<&Adventurer> {
        self.adventurer.as_ref()
    }

    pub fn get_creature(&self) -> Option<&Creature> {
        self.creature.as_ref()
    }

    pub fn set_adventurer(&mut self, adventurer: Adventurer) {
        self.adventurer = Some(adventurer);
    }

    pub fn set_creature(&mut self, creature: Creature) {
        self.creature = Some(creature);
    }

    pub fn remove_adventurer(&mut self) {
        self.adventurer = None;
    }

    pub fn fight(&mut self) {
        if self.adventurer.is_none() || self.creature.is_none() {
            return;
        }

        let mut adventurer = self.adventurer.clone().unwrap();
        let mut creature = self.creature.clone().unwrap();

        let die = Die::new(6);
        let adventurer_roll = die.roll();
        let creature_roll = die.roll();

        if adventurer_roll > creature_roll {
            let damage = (adventurer_roll - creature_roll) as f64;
            creature.set_health(creature.get_health() - damage);
        } else if creature_roll > adventurer_roll {
            let damage = (creature_roll - adventurer_roll) as f64;
            adventurer.set_health(adventurer.get_health() - damage);
        }

        if adventurer.is_alive() && creature.is_alive() {
            adventurer.set_health(adventurer.get_health() - 0.5);
            creature.set_health(creature.get_health() - 0.5);
        }

        if !adventurer.is_alive() {
            adventurer.set_health(0.0);
        }
        if !creature.is_alive() {
            creature.set_health(0.0);
        }

        self.adventurer = Some(adventurer);
        self.creature = Some(creature);
    }

    pub fn print_room(&self) {
        if let Some(a) = &self.adventurer {
            a.print_adventurer();
        }
        if let Some(c) = &self.creature {
            c.print_creature();
        }
    }
}

pub struct Die {
    sides: u32,
}

impl Die {
    pub fn new(sides: u32) -> Self {
        Self { sides }
    }

    pub fn roll(&self) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.sides)
    }
}
