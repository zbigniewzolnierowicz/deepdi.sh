import { FC } from "react";
import { RouterProvider, createBrowserRouter } from "react-router-dom";

const router = createBrowserRouter([
  {
    path: "/",
    element: <div>App</div>,
  },
]);

export const App: FC = () => {
  return <RouterProvider router={router} />;
};
