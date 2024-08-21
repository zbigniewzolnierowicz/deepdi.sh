import { Form, useActionData, useLoaderData, useNavigate, useSubmit } from '@remix-run/react';
import { clsx } from 'clsx';
import { IngredientDTO } from 'common/bindings/IngredientDTO';
import { IngredientUnitDTO } from 'common/bindings/IngredientUnitDTO';
import type { RecipeDTO } from 'common/bindings/RecipeDTO';
import type { SerializedEditorState } from 'lexical';
import { PenLineIcon } from 'lucide-react';
import { Fragment, useEffect, useState } from 'react';
import { Controller, FieldArrayWithId, useFieldArray, UseFieldArrayProps, useForm } from 'react-hook-form';
import { toast } from 'sonner';
import { Centered } from '~/components/centered';
import { Editor } from '~/components/editor';
import { EMPTY_RTE } from '~/components/editor/utils';
import { renderToPlaintext } from '~/components/editor/renderPlaintext';
import { ErrorLine } from '~/components/form/error';
import { Label } from '~/components/form/label';
import { editBorder } from '~/utils/classes';
import { assert } from 'typia';
import { CreateRecipeDTO } from 'common/bindings/CreateRecipeDTO';
import { ActionFunctionArgs } from '@remix-run/node';
import { ButtonAddNew } from '~/components/button';
import { Heading } from '~/components/headings';

export async function action({ request }: ActionFunctionArgs) {
  const data = assert<CreateRecipeDTO>(await request.json());

  const res = await fetch('http://localhost:8111/recipe/create',
    { method: 'POST', body: JSON.stringify(data), headers: { 'Content-Type': 'application/json' } },
  );
  const status = res.status;

  if (status >= 200 && status < 300) {
    const recipe: RecipeDTO = await res.json();
    return {
      status,
      recipe,
    };
  }

  return {
    errorCode: status,
    body: await res.json(),
  };
}

export async function loader() {
  const res = await fetch('http://localhost:8111/ingredient');
  const ingredients: IngredientDTO[] = await res.json();

  return {
    availableIngredients: ingredients,
  };
}

interface InternalIngredientWithAmount {
  ingredient_id?: string | null;
  amount: IngredientUnitDTO;
  optional?: boolean;
  notes?: string;
}

interface RecipeCreateForm {
  name: string;
  description: SerializedEditorState;
  steps: SerializedEditorState[];
  ingredients: InternalIngredientWithAmount[];
}

function makeRequiredAndNotEmpty<T>(
  text: string,
  validateFn: (value: T) => boolean,
  rules?: UseFieldArrayProps['rules'],
) {
  return {
    ...rules,
    required: text,
    validate: (value: T) => {
      if (!validateFn(value)) {
        return text;
      }
    },
  };
}

function validateRTEContent(value: SerializedEditorState | null): boolean {
  return Boolean(value && renderToPlaintext(value)?.trim());
}

const MIN_AMOUNT = 1;

const UNITS: IngredientUnitDTO['_type'][] = ['cups', 'grams', 'other', 'teaspoons', 'mililiters'] as const;

export default function CreateRecipeRoute() {
  const data = useActionData<typeof action>();
  const { availableIngredients } = useLoaderData<typeof loader>();
  const submit = useSubmit();
  const navigate = useNavigate();
  const { register, handleSubmit, control, formState } = useForm<RecipeCreateForm>({
    reValidateMode: 'onSubmit',
    criteriaMode: 'all',
  });
  const steps = useFieldArray({
    name: 'steps',
    control,
    rules: {
      required: `Your recipe should have at least ${MIN_AMOUNT} steps!`,
      minLength: {
        value: MIN_AMOUNT,
        message: `Your recipe should have at least ${MIN_AMOUNT} steps!`,
      },
    },
  });

  const ingredients = useFieldArray({
    name: 'ingredients',
    control,
    /* rules: {
      required: `Your recipe should have at least ${MIN_AMOUNT} ingredients!`,
      minLength: {
        value: MIN_AMOUNT,
        message: `Your recipe should have at least ${MIN_AMOUNT} ingredients!`,
      },
    }, */
  });

  const submitData = (data: RecipeCreateForm) => {
    console.group('SUBMITTING');
    console.log('SUBMITTED', data);
    const payload = assert<CreateRecipeDTO>({
      ingredients: data.ingredients.map(i => ({ ...i, optional: i.optional ?? false, notes: i.notes ?? null })),
      name: data.name,
      time: {},
      steps: data.steps.map(s => JSON.stringify(s)),
      description: JSON.stringify(data.description),
      servings: { exact: 1 },
    } as CreateRecipeDTO);
    console.log('PAYLOAD', payload);
    console.groupEnd();

    submit(JSON.stringify(payload), { method: 'post', action: '/recipe/create', encType: 'application/json' });
  };

  useEffect(() => {
    if (data && 'recipe' in data && data.recipe) {
      toast(`Ingredient "${data?.recipe.name}" was successfully created`, {
        richColors: true,
        action: {
          label: 'Open',
          onClick: () => navigate(`/recipe/${data.recipe.id}`),
        },
      });

      navigate('/recipe');
    }
  }, [data, navigate]);

  const RenderField = ({
    ingField,
    index,
  }: {
    ingField: FieldArrayWithId<RecipeCreateForm, 'ingredients', 'id'>;
    index: number;
  }) => {
    const [isOther, setIsOther] = useState(ingField.amount._type === 'other');
    return (
      <Fragment>
        {formState.errors.ingredients?.[index]?.ingredient_id && (
          <ErrorLine>{formState.errors.ingredients[index].ingredient_id.message}</ErrorLine>
        )}
        <select
          defaultValue=""
          {...register(`ingredients.${index}.ingredient_id`, {
            required: 'You must pick an ingredient',
          })}
        >
          <option disabled value="">Select an ingredient</option>
          {availableIngredients.map(ing => (
            <option value={ing.id} key={ing.id}>{ing.name}</option>
          ))}
        </select>
        <select
          {...register(
            `ingredients.${index}.amount._type`,
            { onChange: (e) => { setIsOther(e.target.value === 'other'); } },
          )}
        >
          {UNITS.map(u => (
            <option value={u} key={u}>{u}</option>
          ))}
        </select>
        {isOther
          ? (
              <>
                <input type="text" {...register(`ingredients.${index}.amount.amount.unit`)} />
                <input
                  type="number"
                  step="any"
                  {...register(`ingredients.${index}.amount.amount.amount`, { valueAsNumber: true })}
                />
              </>
            )
          : (
              <input
                type="number"
                step="any"
                {...register(`ingredients.${index}.amount.amount`, { valueAsNumber: true })}
              />
            )}
        <button type="button" onClick={() => ingredients.remove(index)}>Delete</button>
      </Fragment>
    );
  };

  return (
    <Centered>
      {data && 'errorCode' in data && (
        <div className="bg-red-100 mt-4 p-4 border-solid border-red-300 border-2 rounded">
          <Heading>An error has occured.</Heading>
          <pre>{JSON.stringify(data.body, null, 2)}</pre>
        </div>
      )}
      <Form
        onSubmit={handleSubmit(submitData)}
        className="flex flex-col p-2"
      >
        <div className="flex flex-col" aria-labelledby="name">
          <Label htmlFor="name">Recipe name</Label>
          {formState.errors.name && (
            <ErrorLine>{formState.errors.name.message}</ErrorLine>
          )}
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
              {...register('name', { required: 'The name should not be empty' })}
            />
            <span className="w-6 h-6 ml-2 flex-grow-0" aria-hidden="true">
              <PenLineIcon />
            </span>
          </div>
        </div>

        <div className="flex flex-col mt-4" aria-labelledby="description">
          <Label htmlFor="description">Description</Label>
          {formState.errors.description?.message && (
            <ErrorLine>{formState.errors.description.message}</ErrorLine>
          )}
          <Controller
            name="description"
            control={control}
            rules={makeRequiredAndNotEmpty('Description should not be empty', validateRTEContent)}
            render={function DescriptionField({ field }) {
              return (
                <>
                  <Editor
                    className={clsx('mt-2 prose p-2 outline-none focus-within:bg-background-900', editBorder)}
                    name={field.name}
                    value={field.value}
                    onChange={field.onChange}
                    ref={field.ref}
                  />
                </>
              );
            }}
          />
        </div>

        <Label as="h2" id="ingredients">Ingredients</Label>
        {formState.errors.ingredients?.root && (
          <ErrorLine>{formState.errors.ingredients.root.message}</ErrorLine>
        )}
        <div className="flex flex-col mt-4" aria-labelledby="ingredients">
          {ingredients.fields.map((ingField, i) => (
            <RenderField ingField={ingField} index={i} key={ingField.id} />
          ))}

          <ButtonAddNew
            onClick={() => ingredients.append({ amount: { _type: 'grams', amount: 100 } })}
          >
            + Add a new ingredient
          </ButtonAddNew>
        </div>

        <Label as="h2" className="mt-4">Steps</Label>
        {formState.errors.steps?.root && (
          <ErrorLine>{formState.errors.steps?.root.message}</ErrorLine>
        )}
        {steps.fields.map((stepField, i) => (
          <Controller
            name={`steps.${i}`}
            control={control}
            key={stepField.id}
            rules={makeRequiredAndNotEmpty('This step must not be empty', validateRTEContent)}
            render={function StepField({ field, fieldState }) {
              return (
                <div className="mt-4">
                  <div className="flex flex-row justify-between items-center">
                    <Label className="text-xl" htmlFor={`steps.${i}`}>
                      Step {i + 1}
                    </Label>
                    <button type="button" onClick={() => steps.remove(i)}>Delete</button>
                  </div>
                  {fieldState.error && (
                    <ErrorLine>{fieldState.error.message}</ErrorLine>
                  )}
                  <Editor
                    id={`steps.${i}`}
                    className={clsx('mt-2 prose p-2 outline-none focus-within:bg-background-900', editBorder)}
                    name={field.name}
                    value={field.value}
                    onChange={field.onChange}
                    ref={field.ref}
                  />
                </div>
              );
            }}
          />
        ))}

        <ButtonAddNew
          onClick={() => steps.append(EMPTY_RTE)}
        >
          + Add a new step
        </ButtonAddNew>

        <button
          type="submit"
          className={clsx(
            'bg-background-400 hover:bg-background-500 active:bg-background-400',
            'font-bold text-text-900 font-body uppercase',
            'w-fit ml-auto mt-3 p-2',
            'rounded-lg border-2 border-primary-400',
            'transition-colors',
          )}
        >
          Submit
        </button>
      </Form>
    </Centered>
  );
}
