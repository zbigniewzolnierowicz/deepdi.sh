INSERT INTO recipes
(id, name, description, steps, time, servings, metadata)
VALUES
($1, $2, $3, $4, $5, $6, $7)
RETURNING id;
