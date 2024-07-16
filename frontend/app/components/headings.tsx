import { clsx } from 'clsx';
import type { FC, PropsWithChildren } from 'react';

export type HeadingType = React.DetailedHTMLProps<React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement>;

export const Title: FC<PropsWithChildren<HeadingType>> = ({ children, className, ...props }) => (
  <h1 className={clsx('text-4xl font-heading text-text-50 mb-2', className)} {...props}>
    {children}
  </h1>
);

export const Heading: FC<PropsWithChildren<HeadingType & {
  as?: 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6';
}>> = ({
  as: As = 'h2',
  children,
  className,
  ...props
}) => {
  return (
    <As className={clsx('text-2xl font-heading text-text-50 mb-2', className)} {...props}>
      {children}
    </As>
  );
};
