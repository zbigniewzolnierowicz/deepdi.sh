use async_trait::async_trait;

use crate::domain::entities::ingredient::Ingredient;

#[async_trait]
pub trait IngredientRepository {
    async fn insert(&mut self, ingredient: Ingredient) -> eyre::Result<Ingredient>;
}

pub struct InMemoryIngredientRepository(pub Vec<Ingredient>);

#[async_trait]
impl IngredientRepository for InMemoryIngredientRepository {
    async fn insert(&mut self, ingredient: Ingredient) -> eyre::Result<Ingredient> {
        self.0.push(ingredient.clone());

        Ok(ingredient)
    }
}

impl InMemoryIngredientRepository {
    pub fn new() -> Self {
        Self(vec![])
    }
}

