import { LoaderFunctionArgs, redirect, json } from '@remix-run/node';
import { useLoaderData } from '@remix-run/react';
import { clsx } from 'clsx';
import { IngredientDTO } from 'common/bindings/IngredientDTO';
import { DetailedHTMLProps, FC, HTMLAttributes } from 'react';
import { Centered } from '~/components/centered';
import { DietList } from '~/components/ingredients/diets';

type TitleProps = DetailedHTMLProps<HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement>;

const Title: FC<TitleProps> = ({ children, className, ...props }) => (
  <h1 className={clsx('text-3xl font-heading capitalize', className)} {...props}>{children}</h1>
);

type DescriptionProps = DetailedHTMLProps<HTMLAttributes<HTMLParagraphElement>, HTMLParagraphElement>;
const Description: FC<DescriptionProps> = ({ children, className, ...props }) => (
  <p className={clsx(className)} {...props}>{children}</p>
);

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

export default function IngredientRoute() {
  const { ingredient } = useLoaderData<typeof loader>();
  return (
    <Centered className="p-2">
      <Title>{ingredient.name}</Title>
      <DietList
        className="2xl:absolute top-8 left-[calc(50%_-_768px_+_2rem)] mt-2 2xl:mt-0 w-full 2xl:w-80"
        diets={ingredient.diet_friendly}
      />
      <Description className="mt-4">{ingredient.description}</Description>
    </Centered>
  );
}
