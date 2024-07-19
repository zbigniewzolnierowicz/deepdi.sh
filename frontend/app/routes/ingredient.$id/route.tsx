import type { LoaderFunctionArgs, MetaFunction } from '@remix-run/node';
import { redirect, json } from '@remix-run/node';
import { useLoaderData } from '@remix-run/react';
import type { IngredientDTO } from 'common/bindings/IngredientDTO';
import { Centered } from '~/components/centered';
import { DietList } from '~/components/ingredients/diets';
import { Title } from '~/components/headings';
import { LexicalToReact } from '~/components/editor/renderReact';
import { makeTitle } from '~/utils/makeTitle';

export async function loader({ params }: LoaderFunctionArgs) {
  if (!params.id) return redirect('/');

  const ingredient: IngredientDTO | undefined = await fetch(`http://localhost:8111/ingredient/${params.id}`)
    .then((res) => {
      if (res.status !== 200) {
        return undefined;
      }

      return res.json();
    });

  if (!ingredient) return redirect('/');

  return json({
    id: params.id,
    ingredient,
  });
}

export const meta: MetaFunction<typeof loader> = ({ data }) => [
  { title: makeTitle(data?.ingredient.name) },
];

export default function IngredientRoute() {
  const { ingredient } = useLoaderData<typeof loader>();
  const description = JSON.parse(ingredient.description);

  return (
    <Centered className="p-2">
      <Title>{ingredient.name}</Title>
      <DietList
        className="2xl:absolute top-8 left-[calc(50%_-_768px_+_2rem)] mt-2 2xl:mt-0 w-full 2xl:w-80"
        diets={ingredient.diet_friendly}
      />
      <LexicalToReact data={description} />
    </Centered>
  );
}
