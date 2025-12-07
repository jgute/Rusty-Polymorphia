mod adventurer;
mod creature;
mod room;
mod maze;
mod maze_builder;
mod character_factory;

use crate::character_factory::{CharacterFactory};
use adventurer::Adventurer;
use creature::Creature;
use maze::Maze;

fn main() {
    let adventurer:Adventurer = CharacterFactory::create_adventurer("Bilbo");
    let creature:Creature  = CharacterFactory::create_creature( "Ogre");
    let mut maze = Maze::builder()
    .num_rooms(4)
    .moving_cost(0.25)
    .add_adventurer(adventurer)
    .add_creature(creature)
    .build();

    println!("Welcome to the Maze of Polymorphia!");
    println!("An Adventurer and a Creature roam the maze...\n");

    let mut round = 1;
    loop {
        if maze.check_game_over() {
            break;
        }

        println!("==================== ROUND {} ====================", round);
        maze.print_maze();

        println!("Adventurer moves...");
        maze.move_adventurer();

        if maze.in_same_room() {
            println!("They meet in the same room! A battle begins!");

            for room in &mut maze.rooms {
                if room.get_adventurer().is_some() && room.get_creature().is_some() {
                    room.fight();
                    room.print_room();
                }
            }

        } 
        else {
            println!("No encounter this round. The maze is quiet...");
        }

        println!("==================================================\n");
        round += 1;
    }

    println!("\nGame Over!");
}