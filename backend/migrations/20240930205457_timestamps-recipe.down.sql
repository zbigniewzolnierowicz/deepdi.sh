-- Add down migration script here
ALTER TABLE "ingredients_recipes" DROP COLUMN created_at CASCADE;
ALTER TABLE "ingredients_recipes" DROP COLUMN updated_at CASCADE;

ALTER TABLE "recipes" DROP COLUMN created_at CASCADE;
ALTER TABLE "recipes" DROP COLUMN updated_at CASCADE;
