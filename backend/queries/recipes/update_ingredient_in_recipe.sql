UPDATE ingredients_recipes
SET amount = $3
WHERE ingredient_id = $2
AND recipe_id = $1
