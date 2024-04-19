pub mod base;
pub mod errors;
pub mod in_memory;
pub mod postgres;

#[cfg(test)]
pub use in_memory::InMemoryIngredientRepository;
