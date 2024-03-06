import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { FC, useState } from "react";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { Home } from "./pages/Home";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
]);

export const App: FC = () => {
  const [queryClient] = useState(() => {
      return new QueryClient()
  })
  return (
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  );
};
