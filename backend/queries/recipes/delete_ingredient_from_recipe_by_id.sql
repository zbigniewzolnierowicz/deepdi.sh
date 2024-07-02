DELETE FROM ingredients_recipes
WHERE ingredients_recipes.recipe_id = $1
AND ingredients_recipes.ingredient_id = $2;
