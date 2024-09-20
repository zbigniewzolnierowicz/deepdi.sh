import type { LoaderFunctionArgs, MetaFunction } from '@remix-run/node';
import { json, redirect } from '@remix-run/node';
import { useLoaderData } from '@remix-run/react';
import { type RecipeDTO } from 'common/bindings/RecipeDTO';

import { Recipe } from '~/components/recipe/recipe';
import { Centered } from '~/components/centered';
import { makeTitle } from '~/utils/makeTitle';

export const meta: MetaFunction<typeof loader> = ({ data }) => {
  return [
    { title: makeTitle(data?.recipe.name) },
  ];
};

export async function loader({ params }: LoaderFunctionArgs) {
  if (!params.id) return redirect('/');

  const recipe: RecipeDTO | undefined = await fetch(`http://localhost:8111/recipe/${params.id}`)
    .then((res) => {
      if (res.status !== 200) {
        return undefined;
      }
      return res.json();
    });

  if (!recipe) return redirect('/');

  return json({
    id: params.id,
    recipe,
  });
}

export default function RecipeRoute() {
  const { recipe } = useLoaderData<typeof loader>();

  return (
    <Centered>
      <Recipe recipe={recipe} />
    </Centered>
  );
}
