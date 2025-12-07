use Rusty_Polymorphia::character_factory::{CharacterFactory};
use Rusty_Polymorphia::adventurer::Adventurer;

#[test]
fn test_character_factory() {
    let adventurer: Adventurer = CharacterFactory::create_adventurer("Bilbo");
    
    assert_eq!(adventurer.get_name(), "Bilbo");
    assert_eq!(adventurer.get_health(), 5.0);
}
