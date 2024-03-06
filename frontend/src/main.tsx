import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";
import { instrument } from "./telemetry";

const id = "root";
const root = document.getElementById(id);

if (root === null) throw new Error(`Could not find the ${id} element.`);

instrument();

ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
