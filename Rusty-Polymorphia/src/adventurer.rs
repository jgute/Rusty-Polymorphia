#[derive(Clone)]
pub struct Adventurer {
    name: String,
    health: f64,
}

impl Adventurer {
    // Define the initial health as a constant
    pub const INITIAL_ADVENTURER_HEALTH: f64 = 5.0;

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            health: Self::INITIAL_ADVENTURER_HEALTH,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_health(&self) -> f64 {
        self.health
    }

    pub fn set_health(&mut self, health: f64) {
        self.health = health;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn print_adventurer(&self) {
        println!(
            "Adventurer {} (health: {:.2}) is here",
            self.name, self.health
        );
    }
}

// Factory trait
pub trait CharacterFactory {
    fn create(name: &str) -> Self;
}

// Implement factory trait for Adventurer
impl CharacterFactory for Adventurer {
    fn create(name: &str) -> Self {
        Self::new(name)
    }
}
