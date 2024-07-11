SELECT recipe_id
FROM ingredients_recipes
WHERE ingredient_id = ANY($1);
