CREATE TABLE "recipes" (
  "id" uuid PRIMARY KEY,
  "name" varchar(255) NOT NULL,
  "description" text NOT NULL,
  "steps" varchar(255) array NOT NULL,
  "time" json NOT NULL,
  "servings" json NOT NULL,
  "metadata" json NOT NULL
);

CREATE TABLE "ingredients_recipes" (
  "recipe_id" uuid,
  "ingredient_id" uuid,
  "amount" json NOT NULL,
  "notes" text,
  "optional" bool NOT NULL DEFAULT false,
  PRIMARY KEY ("recipe_id", "ingredient_id")
);

ALTER TABLE "ingredients_recipes" ADD FOREIGN KEY ("recipe_id") REFERENCES "recipes" ("id");

ALTER TABLE "ingredients_recipes" ADD FOREIGN KEY ("ingredient_id") REFERENCES "ingredients" ("id");

