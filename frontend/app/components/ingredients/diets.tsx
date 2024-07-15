import { clsx } from 'clsx';
import type { FC } from 'react';
import Deny from '~/icons/deny.svg?react';
import Allow from '~/icons/check.svg?react';

type Diet = {
  text: string;
};

const Diets: Record<string, Diet> = {
  vegan: {
    text: 'Vegan',
  },
  vegetarian: {
    text: 'Vegetarian',
  },
  gluten_free: {
    text: 'Gluten free',
  },
};

export const DietList: FC<{ diets: string[]; className?: string }> = ({ diets: inputDiets, className }) => {
  const diets = Object.entries(Diets).map(([id, data]) => ({
    id,
    ...data,
    violates: inputDiets.includes(id),
  }));

  return (
    <div className={clsx('mr-2', className)}>
      <div className="font-semibold font-heading border-b-2 pb-2 mb-2 border-background-700">Diets</div>
      <ul>
        {diets.map(diet => (
          <li key={diet.id} className="flex flex-row justify-between items-center mb-1 last:mb-0">
            <span>{diet.text}</span>
            <div>
              {diet.violates
                ? <Allow className="text-green-900" />
                : <Deny className="text-red-900" />}
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
};
