SELECT id, name, description, diet_violations
FROM ingredients
WHERE id = $1;
