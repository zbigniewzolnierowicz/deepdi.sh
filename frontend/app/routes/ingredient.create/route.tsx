import type { ActionFunctionArgs, MetaFunction } from '@remix-run/node';
import { json } from '@remix-run/node';
import { Form, useActionData, useSubmit } from '@remix-run/react';
import { clsx } from 'clsx';
import type { ReactElement } from 'react';
import { Centered } from '~/components/centered';
import { Controller, useForm } from 'react-hook-form';
import type { CreateIngredientDTO } from 'common/bindings/CreateIngredientDTO';
import { assert } from 'typia';
import { Editor } from '~/components/editor';
import type { SerializedEditorState } from 'lexical';
import type { IngredientDTO } from 'common/bindings/IngredientDTO';
import { editBorder } from '~/utils/classes';
import { WheatOffIcon, CarrotIcon, Icon, PenLineIcon } from 'lucide-react';
import { carton } from '@lucide/lab';
import { Label } from '~/components/form/label';
import { CheckboxRow } from '~/components/form/checkbox';
import { renderToPlaintext } from '~/components/editor/renderPlaintext';
import { makeTitle } from '~/utils/makeTitle';

export const meta: MetaFunction = () => [
  { title: makeTitle('Create a new ingredient') },
];

export async function action({ request }: ActionFunctionArgs) {
  const data = await request.formData();

  const parsed = {
    ...Object.fromEntries(data.entries()),
  };

  const ingredient = assert<CreateIngredientDTO>({
    name: parsed.name as string,
    description: parsed.description as string,
    diet_friendly: (parsed.dietFriendly as unknown as string).split(','),
  });

  const createdIngredient: IngredientDTO = await fetch(
    'http://localhost:8111/ingredient/create',
    { method: 'POST', body: JSON.stringify(ingredient), headers: { 'Content-Type': 'application/json' } },
  ).then(res => res.json());

  return json({
    ingredient: createdIngredient,
  });
}

const diets: { id: string; name: string; icon?: ReactElement }[] = [
  {
    id: 'vegan',
    name: 'Vegan',
    icon: <CarrotIcon />,
  },
  {
    id: 'vegetarian',
    name: 'Vegetarian',
    icon: <Icon iconNode={carton} />,
  },
  {
    id: 'gluten_free',
    name: 'Gluten-free',
    icon: <WheatOffIcon />,
  },
];

interface IngredientCreateForm {
  name: string;
  description: SerializedEditorState;
  dietFriendly?: string[];
}

export default function CreateIngredientRoute() {
  const data = useActionData<typeof action>();
  const submit = useSubmit();
  const { register, handleSubmit, control } = useForm<IngredientCreateForm>();

  const submitData = (data: IngredientCreateForm) => {
    const payload: CreateIngredientDTO = {
      ...data,
      description: JSON.stringify(data.description),
    };

    submit({ ...payload }, { method: 'post', action: '/ingredient/create' });
  };

  return (
    <Centered>
      {data && <pre>{JSON.stringify(data, null, 2)}</pre>}
      <Form
        onSubmit={handleSubmit(submitData)}
        className="flex flex-col p-2"
      >
        <div className="flex flex-col" aria-labelledby="name">
          <Label htmlFor="name">Ingredient name</Label>
          <div
            className={clsx([
              'flex flex-row justify-stretch items-end focus-within:bg-background-900', 'pb-2 pr-2',
              editBorder,
            ])}
          >
            <input
              id="name"
              className="text-4xl font-heading bg-transparent flex-grow outline-none"
              placeholder="Name"
              autoComplete="off"
              aria-autocomplete="none"
              required
              {...register('name')}
            />
            <span className="w-6 h-6 ml-2 flex-grow-0" aria-hidden="true">
              <PenLineIcon />
            </span>
          </div>
        </div>
        <fieldset className="flex flex-col mt-4 max-w-xs">
          <Label as="legend" className="mb-2">
            Diet restrictions
            {' '}
            <span className="text-xs font-normal text-text-300">(check means diet-friendly)</span>
          </Label>
          {diets.map(diet => (
            <CheckboxRow key={diet.id} id={diet.id} value={diet.id} className="mb-2 last:mb-0" {...register('dietFriendly')}>
              <span className="mr-2" aria-disabled="true">{diet.icon}</span>
              {diet.name}
            </CheckboxRow>
          ))}
        </fieldset>
        <div className="flex flex-col mt-4" aria-labelledby="description">
          <Label htmlFor="description">Description</Label>
          <Controller
            name="description"
            control={control}
            rules={{
              required: true,
              validate: value => !!renderToPlaintext(value),
            }}
            render={({ field }) => (
              <Editor
                className={clsx('mt-2 prose p-2 outline-none focus-within:bg-background-900', editBorder)}
                name={field.name}
                value={field.value}
                onChange={field.onChange}
              />
            )}
          />
        </div>
        {/* TODO: add error messages */}

        <button type="submit">Submit</button>
      </Form>
    </Centered>
  );
}
