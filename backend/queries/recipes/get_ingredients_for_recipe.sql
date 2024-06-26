SELECT
ir.amount,
ir.notes,
ir.optional,
(
    i.id,
    i.name,
    i.description,
    i.diet_friendly
) as "ingredient!: IngredientModel"
FROM ingredients_recipes AS ir
JOIN ingredients AS i
    ON i.id = ir.ingredient_id
WHERE ir.recipe_id = $1
