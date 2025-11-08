#[derive(Clone)]
pub struct Creature {
    name: String,
    health: f64,
}

impl Creature {
    // Define the initial health as a constant
    pub const INITIAL_CREATURE_HEALTH: f64 = 5.0;

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            health: Self::INITIAL_CREATURE_HEALTH,
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

    pub fn print_creature(&self) {
        println!(
            "Creature {} (health: {:.2}) is here",
            self.name, self.health
        );
    }
}
