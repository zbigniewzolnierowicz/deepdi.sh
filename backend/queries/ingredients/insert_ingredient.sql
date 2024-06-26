INSERT INTO ingredients (id, name, description, diet_friendly)
VALUES ($1, $2, $3, $4)
RETURNING id, name, description, diet_friendly;
