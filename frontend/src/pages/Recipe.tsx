import { FC } from "react";
import { Navigate, useParams } from "react-router-dom";
import { UserInfo } from "../components/UserInfo";
import { useQuery } from "@tanstack/react-query";
import { api } from "../api/base";
import { RecipeDTO } from "common/bindings/RecipeDTO";
import { isAxiosError } from "axios";

export const Recipe: FC = () => {
  const { recipeId } = useParams();
  const { data, error, isLoading } = useQuery({
    queryKey: ["recipe", recipeId],
    queryFn: async () => {
      console.log(recipeId);
      const result = await api.get<RecipeDTO>(`/recipes/get/${recipeId}`);

      return result.data;
    },
    retry: (count, error) => {
      if (isAxiosError(error) && error.request.status === 404) {
        return false;
      }
      if (count >= 5) {
        return true;
      }
      return false;
    },
  });

  if (error && isAxiosError(error) && error.response?.status === 404) {
    return <Navigate to="/list" />;
  }

  return (
    <div className="max-w-screen-md mx-auto p-4">
      <div className="mb-4 relative 2xl:fixed 2xl:top-4 2xl:right-4">
        <UserInfo className="ml-auto" />
      </div>
      {isLoading ? (
        <div>Loading...</div>
      ) : (
        <>
          {data && (
            <div>
              <h1 className="text-4xl font-bold">{data.name}</h1>
              <h2 className="text-3xl font-bold mt-2">Ingredients</h2>
              <ul className="mt-2 ml-4 flex flex-col gap-2">
                {data.ingredients.map((ingredient) => (
                  <li>
                    {ingredient.name}: {ingredient.amount}{" "}
                    {ingredient.unit === "count" ? "" : ingredient.unit}
                  </li>
                ))}
              </ul>
              <h2 className="text-3xl font-bold mt-2">Steps</h2>
              <ol className="list-decimal flex flex-col gap-2 mt-2 ml-4">
                {data.steps.map((step) => (
                  <li>{step}</li>
                ))}
              </ol>
            </div>
          )}
          {error && <pre>{JSON.stringify({ error }, null, 2)}</pre>}
        </>
      )}
    </div>
  );
};
