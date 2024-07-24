import { Form, useActionData, useNavigate, useSubmit } from '@remix-run/react';
import { clsx } from 'clsx';
import type { RecipeDTO } from 'common/bindings/RecipeDTO';
import type { SerializedEditorState } from 'lexical';
import { PenLineIcon } from 'lucide-react';
import { useEffect } from 'react';
import { Controller, useForm } from 'react-hook-form';
import { toast } from 'sonner';
import { Centered } from '~/components/centered';
import { Editor } from '~/components/editor';
import { renderToPlaintext } from '~/components/editor/renderPlaintext';
import { Label } from '~/components/form/label';
import { editBorder } from '~/utils/classes';

export async function action() {
  return {
    recipe: {} as RecipeDTO,
  };
}

interface RecipeCreateForm {
  name: string;
  description: SerializedEditorState;
}

export default function CreateRecipeRoute() {
  const data = useActionData<typeof action>();
  const _submit = useSubmit();
  const navigate = useNavigate();
  const { register, handleSubmit, control } = useForm<RecipeCreateForm>();

  const submitData = (data: RecipeCreateForm) => {
    console.log(data);
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

  return (
    <Centered>
      <Form
        onSubmit={handleSubmit(submitData)}
        className="flex flex-col p-2"
      >
        <div className="flex flex-col" aria-labelledby="name">
          <Label htmlFor="name">Recipe name</Label>
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
