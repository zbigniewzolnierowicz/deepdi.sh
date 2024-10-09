INSERT INTO ingredients (id, name, description, diet_violations)
VALUES ($1, $2, $3, $4)
RETURNING id, name, description, diet_violations;
