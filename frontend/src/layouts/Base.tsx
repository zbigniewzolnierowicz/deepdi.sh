import { FC, PropsWithChildren } from "react";
import { Outlet } from "react-router-dom";

export const Base: FC<PropsWithChildren> = ({ children }) => (
  <div className="min-h-svh block bg-slate-800 text-white">{children}</div>
);

export const BaseLayout: FC = () => (
  <Base>
    <Outlet />
  </Base>
);
