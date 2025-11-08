use rand::Rng;

use crate::adventurer::Adventurer;
use crate::creature::Creature;
use crate::room::Room;

pub struct Maze {
    pub rooms: Vec<Room>,
    pub adventurer: Adventurer,
    pub creature: Creature,
    num_rooms: usize,
    moving_cost: f64,
}

impl Maze {
    pub const NORTHWEST: usize = 0;
    pub const NORTHEAST: usize = 1;
    pub const SOUTHEAST: usize = 2;
    pub const SOUTHWEST: usize = 3;
    pub const OUT_OF_BOUNDS: isize = -1;

    pub fn new(adventurer: Adventurer, creature: Creature) -> Self {
        let num_rooms = 4;
        let mut rooms = vec![Room::new(); num_rooms];
        let mut rng = rand::thread_rng();

        let random_adventurer_room = rng.gen_range(0..num_rooms);
        rooms[random_adventurer_room].set_adventurer(adventurer.clone());

        let random_creature_room = rng.gen_range(0..num_rooms);
        rooms[random_creature_room].set_creature(creature.clone());

        Maze {
            rooms,
            adventurer,
            creature,
            num_rooms,
            moving_cost: 0.25,
        }
    }

    fn one_or_negative_one(&self) -> isize {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            -1
        } else {
            1
        }
    }

    pub fn move_adventurer(&mut self) {
        for i in 0..self.num_rooms {
            if self.rooms[i].get_adventurer().is_some() {
                let dir = self.one_or_negative_one();
                let mut new_index = (i as isize + dir) % self.num_rooms as isize;
                if new_index < 0 {
                    new_index = (self.num_rooms - 1) as isize;
                }

                let adv_clone = self.adventurer.clone();
                self.rooms[new_index as usize].set_adventurer(adv_clone);
                self.rooms[i].remove_adventurer();
                break;
            }
        }

        let new_health = self.adventurer.get_health() - self.moving_cost;
        self.adventurer.set_health(new_health);
    }

    pub fn in_same_room(&self) -> bool {
        self.rooms.iter().any(|r| {
            r.get_adventurer().is_some() && r.get_creature().is_some()
        })
    }

    pub fn print_maze(&self) {
        println!("Northwest:");
        self.rooms[Self::NORTHWEST].print_room();

        println!("Northeast:");
        self.rooms[Self::NORTHEAST].print_room();

        println!("Southwest:");
        self.rooms[Self::SOUTHWEST].print_room();

        println!("Southeast:");
        self.rooms[Self::SOUTHEAST].print_room();
        println!();
    }
}
