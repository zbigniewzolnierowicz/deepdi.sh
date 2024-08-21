import type { RecipeDTO } from 'common/bindings/RecipeDTO';
import type { FC, PropsWithChildren } from 'react';
import { IngredientList } from './ingredientList';
import { formatDuration } from 'date-fns';
import type { ServingsTypeDTO } from 'common/bindings/ServingsTypeDTO';
import { convertSecondsToDuration } from '~/utils/convertSecondsToDuration';
import { Title, Heading } from '~/components/headings';
import { LexicalToReact } from '../editor/renderReact';

const Description: FC<PropsWithChildren> = ({ children }) => (
  <div className="mb-2">
    {children}
  </div>
);

const Step: FC<{ step: string; index: number }> = ({ step, index }) => (
  <>
    <h3 className="text-xl font-heading text-text-50 mb-2">Step {index + 1}</h3>
    <LexicalToReact data={JSON.parse(step)} />
  </>
);

const Steps: FC<{ steps: string[] }> = ({ steps }) => steps.map((step, i) => (
  <Step step={step} index={i} key={i} />
));

const Metadata: FC<{ data: Record<string, string> }> = ({ data }) => (
  <ul className="pl-8 my-4">
    {Object.entries(data).map(([description, value]) => (
      <li key={description}>
        <b>{description}</b>
        {': '}
        {value}
      </li>
    ))}
  </ul>
);

const formatServings = (servings: ServingsTypeDTO): string => {
  if ('exact' in servings) {
    return servings.exact.toString(10);
  }
  else {
    const [lower, higher] = servings.from_to;
    return `between ${lower} and ${higher}`;
  }
};

const convertRecipeTimesToMetadata = (times: RecipeDTO['time']) => {
  const entries = Object.entries(times).map(([type, time]) => {
    return [type, formatDuration(convertSecondsToDuration(Number(time)))];
  });

  return Object.fromEntries(entries);
};

export const Recipe: FC<{ recipe: RecipeDTO }> = ({ recipe }) => {
  const metadata = {
    ...convertRecipeTimesToMetadata(recipe.time),
    Serves: formatServings(recipe.servings),
  };

  return (
    <div className="px-2 font-body">
      <Title>{recipe.name}</Title>
      <Description>
        <LexicalToReact data={JSON.parse(recipe.description)} />
      </Description>
      <Metadata data={metadata} />
      <Heading>Ingredients</Heading>
      <IngredientList className="mb-2" ingredients={recipe.ingredients} />
      <Heading>Steps</Heading>
      <Steps steps={recipe.steps} />
    </div>
  );
};
