import { clsx } from 'clsx';
import type { FC } from 'react';
import * as Tooltip from '@radix-ui/react-tooltip';
import { CircleCheckIcon, CircleXIcon } from 'lucide-react';

import { diets } from './diets';

export const DietList: FC<{ diets: string[]; className?: string }> = ({ diets: inputDiets, className }) => {
  const dietsDisplay = diets.map(diet => ({
    ...diet,
    violates: inputDiets.includes(diet.id),
    description: inputDiets.includes(diet.id) ? diet.friendly : diet.unfriendly,
  }));

  return (
    <div className={clsx('mr-2', className)}>
      <div className="font-semibold font-heading border-b-2 pb-2 mb-2 border-background-700">Diets</div>
      <ul>
        {dietsDisplay.map(diet => (
          <li key={diet.id} className="flex flex-row justify-between items-center mb-2 last:mb-0">
            <span className="flex flex-row">
              <span className="mr-2">{diet.icon}</span>
              {diet.name}
            </span>
            <div>
              <Tooltip.Root>
                <Tooltip.Trigger>
                  {diet.violates
                    ? (
                        <CircleCheckIcon className="text-green-900" />
                      )
                    : (
                        <CircleXIcon className="text-red-900" />
                      )}
                </Tooltip.Trigger>
                <Tooltip.Portal>
                  <Tooltip.Content
                    className={
                      clsx(
                        'p-2 rounded',
                        diet.violates ? 'bg-background-700' : 'bg-red-300',
                      )
                    }
                  >
                    {diet.description}
                    <Tooltip.Arrow
                      className={
                        clsx(
                          diet.violates ? 'fill-background-700' : 'fill-red-300',
                        )
                      }
                    />
                  </Tooltip.Content>
                </Tooltip.Portal>
              </Tooltip.Root>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
};
