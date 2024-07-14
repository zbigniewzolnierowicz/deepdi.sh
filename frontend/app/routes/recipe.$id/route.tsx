import { json, LoaderFunctionArgs, MetaFunction, redirect } from '@remix-run/node';
import { useLoaderData } from '@remix-run/react';
import { type RecipeDTO } from 'common/bindings/RecipeDTO';
import { Recipe } from '~/components/recipe/recipe';
import { Centered } from '~/components/centered';

export const meta: MetaFunction<typeof loader> = ({ data }) => {
  return [
    { title: data?.recipe.name ? `${data.recipe.name} - deepdi.sh` : 'deepdi.sh' },
    { name: 'description', content: 'Welcome to Remix!' },
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
