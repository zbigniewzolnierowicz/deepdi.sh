import { clsx } from 'clsx';
import { forwardRef } from 'react';

type ButtonType = React.DetailedHTMLProps<React.ButtonHTMLAttributes<HTMLButtonElement>, HTMLButtonElement>;
export const ButtonAddNew = forwardRef<HTMLButtonElement, ButtonType>(function ButtonAddNew({ children, type, className, ...props }, ref) {
  return (
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
        className,
      )}
      type={type ?? 'button'}
      ref={ref}
      {...props}
    >
      + Add a new ingredient
    </button>
  );
});
