use std::time::Duration;

use crate::domain::entities::ingredient::{
    types::{DietFriendly, WhichDiets},
    Ingredient,
};
use crate::domain::entities::recipe::{IngredientUnit, IngredientWithAmount, ServingsType};

use super::*;

fn recipe_fixture() -> Recipe {
    // Recipe from https://publicdomainrecipes.com/hoisin_tofu_and_broccoli/
    Recipe {
        id: uuid::Uuid::from_u128(0),
        name: "Hoisin Tofu and Broccoli".to_string(),
        description: "If necessary, provide a very brief description of the dish in one or two sentences. For most dishes, this will be unnecessary. If there is a title image of this dish, it should be above this paragraph. You may also include prep/cook time and the number of servings as below:".to_string(),
        time: HashMap::from([
            ("Prep time".to_string(), Duration::from_secs(15 * 60)),
            ("Cook time".to_string(), Duration::from_secs(10 * 60))
        ]),
        servings: ServingsType::Exact(4),
        ingredients: vec![
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(0),
                    name: "Firm tofu".try_into().unwrap(),
                    description: "It's tofu".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian, DietFriendly::GlutenFree]),
                },
                amount: IngredientUnit::Grams(400.0),
                notes: None,
                optional: false
            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(1),
                    name: "Broccoli".try_into().unwrap(),
                    description: "It's broccoli".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian, DietFriendly::GlutenFree]),
                },
                amount: IngredientUnit::Other("head".to_string(), 1.0),
                notes: None,
                optional: false
            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(2),
                    name: "Garlic".try_into().unwrap(),
                    description: "Garlic description".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian, DietFriendly::GlutenFree]),
                },
                amount: IngredientUnit::Other("clove".to_string(), 4.0),
                notes: None,
                optional: false

            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(3),
                    name: "Light soy sauce".try_into().unwrap(),
                    description: "Soy sauce that is light".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian]),
                },
                amount: IngredientUnit::Cup(0.5),
                notes: None,
                optional: false
            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(4),
                    name: "Dark soy sauce".try_into().unwrap(),
                    description: "Soy sauce that is dark".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian]),
                },
                amount: IngredientUnit::Cup(1.0),
                notes: None,
                optional: false
            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(5),
                    name: "Sugar".try_into().unwrap(),
                    description: "Sugar description".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian, DietFriendly::GlutenFree]),
                },
                amount: IngredientUnit::from_tablespoons(2.0),
                notes: None,
                optional: false
            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(6),
                    name: "Rice vinegar".try_into().unwrap(),
                    description: "Vinegar made out of rice".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian, DietFriendly::GlutenFree]),
                },
                amount: IngredientUnit::from_tablespoons(2.0),
                notes: Some("Or any other non-balsamic vinegar".to_string()),
                optional: false
            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(7),
                    name: "Rice vinegar".try_into().unwrap(),
                    description: "Vinegar made out of rice".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian, DietFriendly::GlutenFree]),
                },
                amount: IngredientUnit::from_tablespoons(2.0),
                notes: None,
                optional: false
            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(8),
                    name: "Vegetable stock".try_into().unwrap(),
                    description: "Stock made out of vegetables".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian, DietFriendly::GlutenFree]),
                },
                amount: IngredientUnit::Cup(0.25),
                notes: Some("or water".to_string()),
                optional: false
            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(9),
                    name: "Vegetable stock".try_into().unwrap(),
                    description: "Stock made out of vegetables".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian, DietFriendly::GlutenFree]),
                },
                amount: IngredientUnit::Cup(0.25),
                notes: None,
                optional: false
            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(10),
                    name: "Shaoxin wine".try_into().unwrap(),
                    description: "Shaoxin wine description".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![]),
                },
                amount: IngredientUnit::from_tablespoons(2.0),
                notes: Some("or dry cherry wine".to_string()),
                optional: true
            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(11),
                    name: "Cornstarch".try_into().unwrap(),
                    description: "Cornstarch desc".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian, DietFriendly::GlutenFree]),
                },
                amount: IngredientUnit::from_tablespoons(0.5),
                notes: None,
                optional: false
            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(12),
                    name: "Sesame oil".try_into().unwrap(),
                    description: "Oil made out of sesame".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian, DietFriendly::GlutenFree]),
                },
                amount: IngredientUnit::from_tablespoons(0.5),
                notes: None,
                optional: false
            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(13),
                    name: "Peanut oil".try_into().unwrap(),
                    description: "Oil made out of peanuts".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian, DietFriendly::GlutenFree]),
                },
                amount: IngredientUnit::from_tablespoons(2.5),
                notes: Some("any oil will do".to_string()),
                optional: false
            },
            IngredientWithAmount {
                ingredient: Ingredient {
                    id: uuid::Uuid::from_u128(14),
                    name: "Ginger".try_into().unwrap(),
                    description: "Spicy root".try_into().unwrap(),
                    diet_friendly: WhichDiets(vec![DietFriendly::Vegan, DietFriendly::Vegetarian, DietFriendly::GlutenFree]),
                },
                amount: IngredientUnit::Grams(75.0),
                notes: None,
                optional: true
            },
        ],
        steps: vec![
            "Cut the tofu into 3/4” (2 cm) cubes and place it in a sealable ziplock bag.".to_string(),
            "Combine 1/4 cup soy sauce, 1/2 teaspoon dark soy sauce, sugar and rice vinegar ingredients in a bowl and stir to mix.".to_string(),
            "A food prep rectangular tupperware could work very well, as you need less marinade to submerge the tofu. Add tofu and refrigirate for half an hour.".to_string(),
            "Combine vegetable stock, remaining soy sause, wine, sesame oil, cornstarch and hoisin sause into a bowl and stir until the cornstarch is dissolved.".to_string(),
            "Cut the florets off the stalks of broccoli. Cut into 1cm chunks. Wash and drain if needed.".to_string(),
            "Heat 1/3 cup of water in a large pan over medium/high heat until boiling. Add broccoli and cover with a lid. Steam for 2-3 minutes depending on how hard you want it, then drain and rinse the remaining water off. Put the broccoli aside.".to_string(),
            "Add 2 tbsp of oil on the same pan. Cook tofu until the bottom is golden brown, flip and repeat. Stir a few times, then move to the side of the pan.".to_string(),
            "Add more oil, garlic, ginger. Stir until it releases fragrance - generally less than a minute, then stir to combine the two.".to_string(),
            "Cook together until sause thickens, then add broccoli and stir to combine.".to_string(),
        ]
    }
}

#[tokio::test]
async fn creating_recipe_works() {
    let repo = InMemoryRecipeRepository::new();

    let recipe = recipe_fixture();
    let result = repo.insert(recipe.clone()).await.unwrap();
    assert_eq!(recipe, result);

    let lock = repo.0.lock().unwrap();
    let inner_result = lock
        .get(&result.id)
        .expect("The recipe wasn't found in the hashmap");

    assert_eq!(inner_result, &recipe.clone());
}

#[tokio::test]
async fn inserting_recipe_with_same_id_fails() {
    let repo = InMemoryRecipeRepository::new();

    let recipe = recipe_fixture();

    repo.insert(recipe.clone()).await.unwrap();

    let error = repo.insert(recipe.clone()).await.unwrap_err();

    assert!(matches!(error, RecipeRepositoryError::Conflict(a) if a == "id"));
}
