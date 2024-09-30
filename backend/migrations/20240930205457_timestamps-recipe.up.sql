-- Add up migration script here
ALTER TABLE "ingredients_recipes" ADD COLUMN created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT timezone('utc', now());
ALTER TABLE "ingredients_recipes" ADD COLUMN updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT timezone('utc', now());

ALTER TABLE "recipes" ADD COLUMN created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT timezone('utc', now());
ALTER TABLE "recipes" ADD COLUMN updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT timezone('utc', now());
