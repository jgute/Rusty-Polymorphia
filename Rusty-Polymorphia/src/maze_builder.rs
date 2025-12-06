use rand::Rng;

use crate::adventurer::Adventurer;
use crate::creature::Creature;
use crate::room::Room;
use crate::maze::Maze;

pub struct MazeBuilder {
    num_rooms: usize,
    moving_cost: f64,
    adventurer: Option<Adventurer>,
    creature: Option<Creature>,
}

impl MazeBuilder {
    pub fn new() -> Self {
        MazeBuilder {
            num_rooms: 4,
            moving_cost: 0.25,
            adventurer: None,
            creature: None,
        }
    }

    pub fn num_rooms(mut self, num_rooms: usize) -> Self  {
        assert!(num_rooms > 0, "Maze must have at least one room");
        self.num_rooms = num_rooms;
        self
    }

    pub fn moving_cost(mut self, cost: f64) -> Self {
        self.moving_cost = cost;
        self
    }

    pub fn add_adventurer(mut self, adventurer: Adventurer) -> Self {
        self.adventurer = Some(adventurer);
        self
    }

    pub fn add_creature(mut self, creature: Creature) -> Self {
        self.creature = Some(creature);
        self
    }

    pub fn build(self) -> Maze {
        let num_rooms = self.num_rooms;

        let mut rooms = vec![Room::new(); num_rooms];
        let mut rng = rand::thread_rng();

        let adventurer = self
            .adventurer
            .expect("The Maze needs an Adventurer");
        let creature = self
            .creature
            .expect("The Maze needs a creature");

        let random_adventurer_room = rng.gen_range(0..num_rooms);
        rooms[random_adventurer_room].set_adventurer(adventurer);

        let random_creature_room = rng.gen_range(0..num_rooms);
        rooms[random_creature_room].set_creature(creature);

        Maze {
            rooms,
            num_rooms,
            moving_cost: self.moving_cost,
        }
    }
}
