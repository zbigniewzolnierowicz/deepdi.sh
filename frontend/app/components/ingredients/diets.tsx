import { carton } from '@lucide/lab';
import { CarrotIcon, Icon, WheatOffIcon } from 'lucide-react';
import type { ReactElement } from 'react';

export const diets: { id: string; name: string; icon?: ReactElement; friendly: string; unfriendly: string }[] = [
  {
    id: 'vegan',
    name: 'Vegan',
    icon: <CarrotIcon />,
    friendly: 'This ingredient is vegan friendly',
    unfriendly: 'This ingredient is not vegan friendly',
  },
  {
    id: 'vegetarian',
    name: 'Vegetarian',
    icon: <Icon iconNode={carton} />,
    friendly: 'This ingredient is vegetarian friendly',
    unfriendly: 'This ingredient is not vegetarian friendly',
  },
  {
    id: 'gluten_free',
    name: 'Gluten-free',
    icon: <WheatOffIcon />,
    friendly: 'This ingredient is gluten free.',
    unfriendly: 'This ingredient contains gluten.',
  },
];
