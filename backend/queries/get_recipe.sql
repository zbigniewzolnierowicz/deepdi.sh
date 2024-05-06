SELECT
r.id,
r.name,
r.description,
r.steps,
r.time,
r.servings
FROM recipes AS r
JOIN ingredients_recipes AS ir ON r.id = ir.recipe_id
JOIN ingredients AS i ON ir.ingredient_id = i.id
WHERE r.id = $1

