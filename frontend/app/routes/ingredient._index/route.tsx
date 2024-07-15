import { Link, useLoaderData } from '@remix-run/react';
import type { IngredientDTO } from 'common/bindings/IngredientDTO';
import { Centered } from '~/components/centered';
import { Title } from '~/components/headings';

export async function loader() {
  const ingredients: IngredientDTO[] = await fetch('http://localhost:8111/ingredient').then(res => res.json());

  return {
    ingredients,
  };
}

export default function IngredientList() {
  const { ingredients } = useLoaderData<typeof loader>();

  return (
    <Centered>
      <Title>Ingredients</Title>
      {ingredients.map(ingredient => (
        <ul key={ingredient.id}>
          <li>
            <Link to={`/ingredient/${ingredient.id}`}>{ingredient.name}</Link>
          </li>
        </ul>
      ))}
    </Centered>
  );
}
