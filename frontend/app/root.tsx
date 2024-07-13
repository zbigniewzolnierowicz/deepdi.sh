import {
  Links,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from '@remix-run/react';

import './tailwind.css';
import '@fontsource-variable/raleway';
import '@fontsource-variable/playfair-display';
import { FC, PropsWithChildren } from 'react';
import * as Tooltip from '@radix-ui/react-tooltip';

export function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body>
        {children}
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}

const Providers: FC<PropsWithChildren> = ({ children }) => (
  <Tooltip.Provider>
    {children}
  </Tooltip.Provider>
);

export default function App() {
  return (
    <Providers>
      <div className="bg-background-950">
        <Outlet />
      </div>
    </Providers>
  );
}
