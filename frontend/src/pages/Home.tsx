import { FC } from "react";
import { Link } from "react-router-dom";

export const Home: FC = () => {
  return (
    <div className="bg-zinc-200 text-black w-full min-h-svh flex flex-col items-center justify-center">
      <h1 className="text-6xl font-bold bg-gradient-to-r from-green-400 to-yellow-400 text-transparent bg-clip-text">
        Welcome to Rustipe!
      </h1>
      <h2 className="text-3xl">A modern recipe listing page</h2>
      <div>
        <Link to="/list">All recipes</Link>
      </div>
    </div>
  );
};
