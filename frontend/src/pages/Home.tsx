import { useInfiniteQuery } from "@tanstack/react-query";
import clsx from "clsx";
import { RecipeDTO } from "common/bindings/RecipeDTO";
import { FC, useEffect } from "react";
import { api } from "../api/base";
import { UserInfo } from "../components/UserInfo";
import { useLoginState } from "../stores/login";

const RecipePreview: FC<{ recipe: RecipeDTO }> = ({ recipe }) => (
  <div
    className={clsx(
      "bg-slate-700 first:mt-0 my-2 p-4 rounded-md shadow shadow-gray-700",
    )}
  >
    <h2 className="text-4xl mb-4">{recipe.name}</h2>
    <div className="w-full h-auto aspect-video bg-slate-800" />
    <p className="mt-4 text-lg">{recipe.description}</p>
  </div>
);

const RecipeList: FC = () => {
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
      retry: 3,
      getNextPageParam: (lastPage, allPages) => {
        const nextPage: number | undefined =
          lastPage.length === COUNT ? allPages.flat().length : undefined;

        return nextPage;
      },
    },
  );

  return (
    <div className="flex flex-col gap-2">
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
    </div>
  );
};

const COUNT = 3;

export const Home: FC = () => {
  const { fetchUserData } = useLoginState();
  const { userData, errors } = useLoginState();

  useEffect(() => {
    fetchUserData();
  }, [fetchUserData]);

  return (
    <div className="min-h-svh dark:bg-slate-800 text-white">
      <div className="max-w-screen-md mx-auto p-2">
        <div className="fixed top-4 right-4 w-fit">
          <UserInfo />
        </div>

        {errors !== null && errors.length > 0 ? (
          <div>
            The following errors have occursed:{" "}
            <ul>
              {errors?.map((e) => (
                <li>{e.message}</li>
              ))}
            </ul>
          </div>
        ) : userData !== null ? (
          <RecipeList />
        ) : (
          <div>You must be logged in.</div>
        )}
      </div>
    </div>
  );
};
