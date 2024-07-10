use std::{collections::HashMap, sync::OnceLock};

/// Turns out Postgres doesn't return the column name for unique constraints isn't returned.
/// This function maps constraints to fields
fn constraint_to_field(field: &str) -> &str {
    static HASHMAP: OnceLock<HashMap<&str, &str>> = OnceLock::new();
    let m = HASHMAP.get_or_init(|| {
        HashMap::from_iter([
            ("ingredients_name_key", "ingredient name"),
            ("ingredients_pkey", "ingredient id"),
            ("recipes_pkey", "recipe id"),
        ])
    });
    m.get(field).unwrap_or(&field)
}

mod delete;
mod get;
mod ingredients;
mod insert;
mod update;

pub use delete::*;
pub use get::*;
pub use ingredients::*;
pub use insert::*;
pub use update::*;
