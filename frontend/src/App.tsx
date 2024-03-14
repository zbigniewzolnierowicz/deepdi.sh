import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { FC, useEffect, useState } from "react";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { BaseLayout } from "./layouts/Base";
import { Home } from "./pages/Home";
import { ListPage } from "./pages/List";
import { Signup } from "./pages/Signup";
import { useLoginState } from "./stores/login";
import { Recipe } from "./pages/Recipe";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
  {
    path: "/",
    element: <BaseLayout />,
    children: [
      {
        path: "/list",
        element: <ListPage />,
      },
      {
        path: "/signup",
        element: <Signup />,
      },
      {
        path: "/recipe/:recipeId",
        element: <Recipe />
      }
    ],
  },
]);

export const App: FC = () => {
  const [queryClient] = useState(() => {
    return new QueryClient();
  });

  const { fetchUserData } = useLoginState()

  useEffect(() => {
    fetchUserData()
  }, [fetchUserData])

  return (
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  );
};
