mod adventurer;
mod creature;
mod room;
mod maze;
mod maze_builder;

use adventurer::Adventurer;
use creature::Creature;
use maze::Maze;
use maze_builder::MazeBuilder;
use std::{thread, time::Duration};

fn main() {
    let adventurer = Adventurer::new("Bilbo");
    let creature = Creature::new("Ogre");
    let mut maze = Maze::builder()
    .num_rooms(4)
    .moving_cost(0.25)
    .add_adventurer(adventurer)
    .add_creature(creature)
    .build();

    println!("Welcome to the Maze of Polymorphia!");
    println!("An Adventurer and a Creature roam the maze...\n");

    // Main game loop
    let mut round = 1;
    loop {
        println!("==================== ROUND {} ====================", round);
        maze.print_maze();

        // Move the adventurer
        println!("Adventurer moves...");
        maze.move_adventurer();

        // Check if they meet
        if maze.in_same_room() {
            println!("They meet in the same room! A battle begins!");

            for room in &mut maze.rooms {
                if room.get_adventurer().is_some() && room.get_creature().is_some() {
                    room.fight();
                    room.print_room();
                }
            }

            if maze.check_game_over() {
                break;
            }
        } else {
            println!("No encounter this round. The maze is quiet...");
        }

        println!("==================================================\n");
        round += 1;

        // End condition if it drags too long
        if round > 50 {
            println!("The maze grows silent... the hunt goes on forever.");
            break;
        }
    }

    println!("\nGame Over!");
}
