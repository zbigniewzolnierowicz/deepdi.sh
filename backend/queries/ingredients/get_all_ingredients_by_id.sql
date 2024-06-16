SELECT id, name, description, diet_friendly
FROM ingredients
WHERE id = ANY($1);
