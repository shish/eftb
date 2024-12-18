import { createRootRoute, Link, Outlet } from "@tanstack/react-router";

import "../index.css";
import { Suspense } from "react";
import React from "react";
const TanStackRouterDevtools =
  // eslint-disable-next-line
  process.env.NODE_ENV === "production"
    ? () => null // Render nothing in production
    : React.lazy(() =>
        import("@tanstack/router-devtools").then((res) => ({
          default: res.TanStackRouterDevtools,
        })),
      );

export const Route = createRootRoute({
  component: () => (
    <>
      <h1>EVE Frontier Toolbox</h1>
      <p>
        Very much a work in progress, let Shish know if something seems off.
        Also let Shish know if things seem accurate TBH, right now a lot of this
        stuff is theoretical and I&apos;d love to have some confirmation that it
        is working for people :)
      </p>
      <hr />
      <Outlet />
      <hr />
      <ul>
        <li>
          <Link to="/" className="[&.active]:font-bold">
            Home
          </Link>
        </li>
        <li>
          <Link to="/calc/jump" className="[&.active]:font-bold">
            Ship Jump Capability
          </Link>{" "}
          - How far can I jump?
        </li>
        <li>
          <Link to="/calc/dist" className="[&.active]:font-bold">
            Distance Between Systems
          </Link>{" "}
          - How far do I need to jump?
        </li>
        <li>
          <Link to="/calc/path" className="[&.active]:font-bold">
            Jump Route Planner
          </Link>{" "}
          - How do I jump from A to B in several hops?
        </li>
        <li>
          <Link to="/calc/fuel" className="[&.active]:font-bold">
            Fuel Requirement
          </Link>{" "}
          - How much fuel do I need to jump that far?
        </li>
        <li>
          <Link to="/calc/exit" className="[&.active]:font-bold">
            Region Exit Finder
          </Link>{" "}
          - How do I get out of a trapped region?
        </li>
        <li>
          <a href="https://ko-fi.com/shish2k">Buy me a coffee?</a>
        </li>
        {/*
        <li>
          <Link to="/about" className="[&.active]:font-bold">
            About
          </Link>
        </li>
         */}
      </ul>
      <Suspense>
        <TanStackRouterDevtools />
      </Suspense>
    </>
  ),
});
