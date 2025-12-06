use Rusty_Polymorphia::adventurer::Adventurer;
use Rusty_Polymorphia::adventurer::CharacterFactory;

#[test]
fn test_character_factory() {
    let bilbo = Adventurer::create("Bilbo");
    
    print!("Created adventurer: {} with health {:.2}\n", bilbo.get_name(), bilbo.get_health());

    assert_eq!(bilbo.get_name(), "Bilbo");
    assert_eq!(bilbo.get_health(), 5.0);
}