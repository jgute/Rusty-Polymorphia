use rand::Rng;

use crate::adventurer::Adventurer;
use crate::creature::Creature;
use crate::room::Room;
use crate::maze_builder::MazeBuilder;


pub struct Maze {
    pub rooms: Vec<Room>,
    pub num_rooms: usize,
    pub moving_cost: f64,
}

impl Maze {
    pub const NORTHWEST: usize = 0;
    pub const NORTHEAST: usize = 1;
    pub const SOUTHEAST: usize = 2;
    pub const SOUTHWEST: usize = 3;
    pub const OUT_OF_BOUNDS: isize = -1;

    pub fn new(adventurer: Adventurer,creature: Creature) -> Self {
        Maze::builder()
        .add_adventurer(adventurer)
        .add_creature(creature)
        .build()
    }

    pub fn builder() -> MazeBuilder {
        MazeBuilder::new()
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
            if let Some(a_ref) = self.rooms[i].get_adventurer() {
                if a_ref.get_health() <= 0.0 {
                    return;
                }

                // Clone the adventurer out of the room
                let mut adventurer = a_ref.clone();

                // Apply movement cost to the adventurer's health
                let new_health = adventurer.get_health() - self.moving_cost;
                adventurer.set_health(new_health);

                  // Remove from the old room
                  self.rooms[i].remove_adventurer();

                // Compute new index
                let dir = self.one_or_negative_one();
                let mut new_index = (i as isize + dir) % self.num_rooms as isize;
                if new_index < 0 {
                    new_index = (self.num_rooms - 1) as isize;
                }

                // Place adventurer in new room
                self.rooms[new_index as usize].set_adventurer(adventurer);
                break;
            }
        }
    }

    pub fn in_same_room(&self) -> bool {
        self.rooms.iter().any(|room| {
            room.get_adventurer().is_some() && room.get_creature().is_some()
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

    pub fn check_game_over(&self) -> bool {
        let mut adventurer_dead = false;
        let mut creature_dead = false;

        for room in &self.rooms {
            if let Some(a) = room.get_adventurer() {
                if a.get_health() <= 0.0 {
                    adventurer_dead = true;
                }
            }
            if let Some(c) = room.get_creature() {
                if c.get_health() <= 0.0 {
                    creature_dead = true;
                }
            }
        }

        if adventurer_dead {
            println!("The Adventurer has fallen! The Creature wins!");
        }
        if creature_dead {
            println!("The Creature has been slain! The Adventurer triumphs!");
        }

        adventurer_dead || creature_dead
    }

    /// (Optional helper) Find which room currently has the adventurer.
    pub fn adventurer_room_index(&self) -> Option<usize> {
        for (i, room) in self.rooms.iter().enumerate() {
            if room.get_adventurer().is_some() {
                return Some(i);
            }
        }
        None
    }
}
