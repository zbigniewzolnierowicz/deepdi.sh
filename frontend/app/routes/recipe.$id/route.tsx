import { json, LoaderFunctionArgs, MetaFunction } from '@remix-run/node';
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
  const recipe: RecipeDTO = {
    id: params.id ?? '00000000-0000-0000-0000-000000000001',
    name: 'Deep fried lettuce',
    description: 'This is a recipe I am doing for testing',
    steps: [
      'Do a test',
    ],
    time: {
      'Prep time': 86400 + 3600 + 60 + 59 as unknown as bigint,
    },
    ingredients: [
      {
        ingredient: {
          id: '00000000-0000-0000-0000-000000000001',
          name: 'cucumber',
          description: 'Not a cumcuber',
          diet_friendly: [],
        },
        amount: {
          _type: 'grams',
          amount: 1000000,
        },
        optional: false,
        notes: 'Warning: do not use cumcubers',
      },
      {
        ingredient: {
          id: '00000000-0000-0000-0000-000000000002',
          name: 'Flour',
          description: 'Flour',
          diet_friendly: [],
        },
        amount: {
          _type: 'cups',
          amount: 1,
        },
        optional: false,
        notes: null,
      },
      {
        ingredient: {
          id: '00000000-0000-0000-0000-000000000003',
          name: 'Oil',
          description: 'Oil',
          diet_friendly: [],
        },
        amount: {
          _type: 'teaspoons',
          amount: 4,
        },
        optional: false,
        notes: null,
      },
      {
        ingredient: {
          id: '00000000-0000-0000-0000-000000000004',
          name: 'Lettuce',
          description: 'Lettuce',
          diet_friendly: [],
        },
        amount: {
          _type: 'other',
          amount: {
            unit: 'head',
            amount: 2,
          },
        },
        optional: true,
        notes: null,
      },
    ],
    servings: {
      from_to: [
        2,
        4,
      ],
    },
  };
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
