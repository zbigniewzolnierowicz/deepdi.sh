import { clsx } from 'clsx';
import { FC, PropsWithChildren } from 'react';

export type HeadingType = React.DetailedHTMLProps<React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement>;

export const Title: FC<PropsWithChildren<HeadingType>> = ({ children, className, ...props }) => (
  <h1 className={clsx('text-3xl font-heading text-text-50 mb-2', className)} {...props}>
    {children}
  </h1>
);

export const Heading: FC<PropsWithChildren<HeadingType>> = ({
  children,
  className,
  ...props
}) => (
  <h2 className={clsx('text-2xl font-heading text-text-50 mb-2', className)} {...props}>
    {children}
  </h2>
);
