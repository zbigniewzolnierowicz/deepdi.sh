import { useInfiniteQuery } from "@tanstack/react-query";
import { RecipeDTO } from "common/bindings/RecipeDTO";
import { FC, useEffect } from "react";
import { api } from "../api/base";
import { UserInfo } from "../components/UserInfo";
import { useLoginState } from "../stores/login";

const RecipePreview: FC<{ recipe: RecipeDTO }> = ({ recipe }) => (
  <div className="bg-slate-700 first:mt-0 my-2 p-4">
    <h2 className="text-4xl mb-4">{recipe.name}</h2>
    <div className="w-full h-auto aspect-video bg-slate-800" />
    <p className="mt-4 text-lg">{recipe.description}</p>
  </div>
);

const COUNT = 3;

export const Home: FC = () => {
  const { fetchUserData, userData } = useLoginState();

  const { data, error, isError, fetchNextPage, hasNextPage } = useInfiniteQuery(
    {
      queryKey: ["recipes"],
      queryFn: async ({ pageParam, signal }) => {
        const params = new URLSearchParams();

        params.append("offset", pageParam.toString());
        params.append("count", COUNT.toString());

        const response = await api.get<RecipeDTO[]>("/recipes/get/", {
          signal,
          params,
          responseType: "json",
        });

        if (response.data.length === 0) throw new Error("Ran out of recipes.");

        return response.data;
      },
      initialPageParam: 0,
      getNextPageParam: (lastPage, allPages) => {
        const nextPage: number | undefined =
          lastPage.length === COUNT ? allPages.flat().length : undefined;

        return nextPage;
      },
      enabled: userData !== null,
    },
  );

  useEffect(() => {
    fetchUserData();
  }, [fetchUserData]);

  return (
    <div className="min-h-svh dark:bg-slate-800 text-white">
      <div className="max-w-screen-md mx-auto p-2">
        <div className="fixed top-4 right-4 w-fit">
          <UserInfo />
        </div>

        <div className="mt-2">
          {userData !== null ? (
            <>
              {data?.pages.flat().map((recipe) => (
                <RecipePreview recipe={recipe} key={recipe.id} />
              ))}
              {hasNextPage && (
                <button
                  type="button"
                  onClick={() => fetchNextPage({ cancelRefetch: true })}
                >
                  Next
                </button>
              )}
            </>
          ) : (
            "You must log in"
          )}
          {isError && <pre>{JSON.stringify(error, null, 2)}</pre>}
        </div>
      </div>
    </div>
  );
};
