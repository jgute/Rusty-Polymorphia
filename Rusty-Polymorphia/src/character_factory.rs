pub trait CharacterFactory {
    fn create(name: &str) -> Self;
}