import { Link } from '@remix-run/react';
import { clsx } from 'clsx';

export function TopBar() {
  return (
    <div
      className={
        clsx(
          'w-full h-16',
          'border-b-2 border-primary-400',
          'flex flex-row items-center',
          'px-4',
        )
      }
    >
      <Link
        to="/"
        className="flex flex-shrink-0 flex-grow-0 w-fit h-full border-r-2 border-inherit p-2 pr-6 items-center"
      >
        <span className="w-fit h-fit text-xl font-heading font-bold text-center">
          deepdi.sh
        </span>
      </Link>
    </div>
  );
}
