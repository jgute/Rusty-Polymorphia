mod adventurer;
mod creature;
mod room;
mod maze;

use adventurer::Adventurer;
use creature::Creature;
use maze::Maze;
use std::{thread, time::Duration};

fn main() {
    let adventurer = Adventurer::new("Bilbo");
    let creature = Creature::new("Ogre");
    let mut maze = Maze::new(adventurer, creature);

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

            // Get updated healths
            let adv_health = maze.adventurer.get_health();
            let creature_health = maze.creature.get_health();

            if adv_health <= 0.0 {
                println!("The Adventurer has fallen! The Creature wins!");
                break;
            }
            if creature_health <= 0.0 {
                println!("The Creature has been slain! The Adventurer triumphs!");
                break;
            }
        } else {
            println!("No encounter this round. The maze is quiet...");
        }

        println!("==================================================\n");
        round += 1;

        // Slight pause to make the output readable
        thread::sleep(Duration::from_millis(1200));

        // End condition if it drags too long
        if round > 25 {
            println!("The maze grows silent... the hunt goes on forever.");
            break;
        }
    }

    println!("\nGame Over!");
}
