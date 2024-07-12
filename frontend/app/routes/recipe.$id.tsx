import { json, LoaderFunctionArgs } from '@remix-run/node';
import { Link, useLoaderData } from '@remix-run/react';
import { type RecipeDTO } from 'common/bindings/RecipeDTO';
import { FC, PropsWithChildren } from 'react';
import clsx from 'clsx';
import { IngredientUnitDTO } from 'common/bindings/IngredientUnitDTO';
import { IngredientWithAmountDTO } from 'common/bindings/IngredientWithAmountDTO';
import convert from 'convert-units';
import { formatQuantity } from 'format-quantity';

export async function loader({ params }: LoaderFunctionArgs) {
  const recipe: RecipeDTO = {
    id: params.id ?? '00000000-0000-0000-0000-000000000001',
    name: 'Testing recipe',
    description: 'This is a recipe I am doing for testing',
    steps: [
      'Do a test',
    ],
    time: {
      'Prep time': 1200 as unknown as bigint,
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
          amount: 100,
        },
        optional: false,
        notes: null,
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

const Centered: FC<PropsWithChildren<{ className?: string }>> = ({ className, children }) => {
  return (
    <div className={clsx('max-w-prose mx-auto', className)}>
      {children}
    </div>
  );
};

const mapIngredientUnitToString = (ingredientUnit: IngredientUnitDTO) => {
  let unit: { unit: string; val: number };

  switch (ingredientUnit._type) {
    case 'cups':
      unit = convert(ingredientUnit.amount).from('cup').toBest();
      break;
    case 'grams':
      unit = convert(ingredientUnit.amount).from('g').toBest();
      break;
    case 'teaspoons':
      unit = convert(ingredientUnit.amount).from('tsp').toBest({ exclude: ['mm3', 'cm3', 'm3', 'km3', 'in3', 'fl-oz', 'pnt', 'qt', 'gal', 'ft3', 'yd3'], cutOffNumber: 1 });
      break;
    case 'mililiters':
      unit = convert(ingredientUnit.amount).from('ml').toBest();
      break;
    default:
      unit = {
        unit: ingredientUnit.amount.unit,
        val: ingredientUnit.amount.amount,
      };
  }

  return unit ? `${formatQuantity(unit.val, true)} ${unit.unit}` : '';
};

function Ingredients({ children }: PropsWithChildren) {
  return (
    <ul className="flex flex-col w-full border-background-800 border-solid border rounded shadow-primary-900 shadow-md">
      {children}
    </ul>
  );
}

function IngredientsItem({ ingredient }: { ingredient: IngredientWithAmountDTO }) {
  return (
    <li className="last:border-none border-b-2 border-background-900 hover:bg-background-900 transition-colors">
      <Link className="flex flex-row justify-between font-body p-2" to={`/ingredient/${ingredient.ingredient.id}`}>
        <div className="capitalize">
          {ingredient.ingredient.name}
        </div>
        <p>
          {mapIngredientUnitToString(ingredient.amount)}
        </p>
      </Link>
      {' '}
    </li>
  );
}

export default function Recipe() {
  const data = useLoaderData<typeof loader>();
  return (
    <div className="bg-background-950">
      <Centered>
        <h1 className="text-3xl font-heading text-text-50 mb-2">
          {data.recipe.name}
        </h1>
        <p className="font-body mb-2">
          {data.recipe.description}
        </p>
        <Ingredients>
          {data.recipe.ingredients.map(ingredientWithAmount => (
            <IngredientsItem ingredient={ingredientWithAmount} key={ingredientWithAmount.ingredient.id} />
          ))}
        </Ingredients>
      </Centered>
      <pre>
        {JSON.stringify(data.recipe, null, 2)}
      </pre>
    </div>
  );
}
