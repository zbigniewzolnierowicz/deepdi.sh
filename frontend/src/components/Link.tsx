import clsx from "clsx";
import { FC, PropsWithChildren } from "react";
import { Link as LinkRRD, LinkProps } from "react-router-dom";

export const Link: FC<PropsWithChildren<LinkProps>> = ({
  children,
  className,
  ...props
}) => (
  <LinkRRD
    className={clsx(
      "text-white text-underline",
      "bg-slate-900",
      "shadow hover:shadow-slate-700 transition-shadow",
      "p-3 m-5 rounded-lg",
      className,
    )}
    {...props}
  >
    {children}
  </LinkRRD>
);
