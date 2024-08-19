import { Form, useActionData, useLoaderData, useNavigate, useSubmit } from '@remix-run/react';
import { clsx } from 'clsx';
import { IngredientDTO } from 'common/bindings/IngredientDTO';
import { IngredientUnitDTO } from 'common/bindings/IngredientUnitDTO';
import type { RecipeDTO } from 'common/bindings/RecipeDTO';
import type { SerializedEditorState } from 'lexical';
import { PenLineIcon } from 'lucide-react';
import { Fragment, useEffect, useState } from 'react';
import { Controller, FieldArrayWithId, useFieldArray, useForm } from 'react-hook-form';
import { toast } from 'sonner';
import { Centered } from '~/components/centered';
import { Editor } from '~/components/editor';
import { EMPTY_RTE } from '~/components/editor/utils';
import { renderToPlaintext } from '~/components/editor/renderPlaintext';
import { ErrorLine } from '~/components/form/error';
import { Label } from '~/components/form/label';
import { editBorder } from '~/utils/classes';

export async function action() {
  return {
    recipe: {} as RecipeDTO,
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

function makeRequiredAndNotEmpty<T>(text: string, validateFn: (value: T) => boolean) {
  return {
    required: text,
    validate: (value: T) => {
      if (
        validateFn(value)
      ) {
        return;
      }
      return text;
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
  const _submit = useSubmit();
  const navigate = useNavigate();
  const { register, handleSubmit, control, formState } = useForm<RecipeCreateForm>({
    reValidateMode: 'onSubmit',
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
  });

  const submitData = (data: RecipeCreateForm) => {
    console.log('SUBMITTED', data);
  };

  useEffect(() => {
    if (data?.recipe) {
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

  const RenderField = ({ ingField, index }: { ingField: FieldArrayWithId<RecipeCreateForm, 'ingredients', 'id'>; index: number }) => {
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

        <div className="flex flex-col mt-4" aria-labelledby="ingredients">
          <Label as="h2" id="ingredients">Ingredients</Label>

          {ingredients.fields.map((ingField, i) => (
            <RenderField ingField={ingField} index={i} key={ingField.id} />
          ))}

          <button
            className={clsx(
              'w-full h-20',
              'border-dashed rounded-2xl border-4',
              'border-background-700 hover:border-background-400 focus:border-background-400',
              'bg-background-950 hover:bg-background-900 focus:bg-background-900',
              'my-4',
              'uppercase font-extrabold',
              'text-text-400 hover:text-text-300 focus:text-text-300',
              'transition-colors',
              'outline-none',
            )}
            onClick={() => ingredients.append({ amount: { _type: 'grams', amount: 100 } })}
            type="button"
          >
            + Add a new ingredient
          </button>
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

        <button
          className={clsx(
            'w-full h-20',
            'border-dashed rounded-2xl border-4',
            'border-background-700 hover:border-background-400 focus:border-background-400',
            'bg-background-950 hover:bg-background-900 focus:bg-background-900',
            'my-4',
            'uppercase font-extrabold',
            'text-text-400 hover:text-text-300 focus:text-text-300',
            'transition-colors',
            'outline-none',
          )}
          onClick={() => steps.append(EMPTY_RTE)}
          type="button"
        >
          + Add a new step
        </button>

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
