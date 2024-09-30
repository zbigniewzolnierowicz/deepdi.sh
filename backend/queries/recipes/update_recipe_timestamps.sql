UPDATE recipes
SET updated_at = timezone('utc', now())
WHERE recipes.id = $1;
