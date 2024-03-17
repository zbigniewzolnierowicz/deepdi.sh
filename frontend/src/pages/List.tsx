import { useInfiniteQuery } from "@tanstack/react-query";
import clsx from "clsx";
import { RecipeDTO } from "common/bindings/RecipeDTO";
import { FC } from "react";
import { api } from "../api/base";
import { UserInfo } from "../components/UserInfo";
import { Link } from "react-router-dom";

const COUNT = 3;

const RecipePreview: FC<{ recipe: RecipeDTO }> = ({ recipe }) => (
  <Link
    className={clsx(
      "bg-zinc-200 first:mt-0 my-2 p-4 border border-black",
    )}
    to={`/recipe/${recipe.id}`}
  >
    <h2 className="text-3xl mb-4">{recipe.name}</h2>
    <div className="w-full h-auto aspect-video bg-zinc-300 border border-black" />
    <p className="mt-4 text-lg">{recipe.description}</p>
  </Link>
);

const RecipeList: FC = () => {
  const { data, fetchNextPage, hasNextPage } = useInfiniteQuery({
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
  });

  return (
    <div className="flex flex-col gap-2 text-black">
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

export const ListPage: FC = () => {
  return (
    <div className="max-w-screen-md mx-auto p-4">
      <div className="mb-4 relative 2xl:fixed 2xl:sidewinder-left-screen-md">
        <UserInfo className="ml-auto" />
      </div>
      <RecipeList />
    </div>
  );
};
