/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file was automatically generated by TanStack Router.
// You should NOT make any changes in this file as it will be overwritten.
// Additionally, you should also exclude this file from your linter and/or formatter to prevent it from being checked or modified.

import { createFileRoute } from '@tanstack/react-router'

// Import Routes

import { Route as rootRoute } from './routes/__root'
import { Route as IndexImport } from './routes/index'
import { Route as CalcPathImport } from './routes/calc/path'
import { Route as CalcJumpImport } from './routes/calc/jump'
import { Route as CalcFuelImport } from './routes/calc/fuel'
import { Route as CalcExitImport } from './routes/calc/exit'
import { Route as CalcDistImport } from './routes/calc/dist'

// Create Virtual Routes

const AboutLazyImport = createFileRoute('/about')()

// Create/Update Routes

const AboutLazyRoute = AboutLazyImport.update({
  id: '/about',
  path: '/about',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/about.lazy').then((d) => d.Route))

const IndexRoute = IndexImport.update({
  id: '/',
  path: '/',
  getParentRoute: () => rootRoute,
} as any)

const CalcPathRoute = CalcPathImport.update({
  id: '/calc/path',
  path: '/calc/path',
  getParentRoute: () => rootRoute,
} as any)

const CalcJumpRoute = CalcJumpImport.update({
  id: '/calc/jump',
  path: '/calc/jump',
  getParentRoute: () => rootRoute,
} as any)

const CalcFuelRoute = CalcFuelImport.update({
  id: '/calc/fuel',
  path: '/calc/fuel',
  getParentRoute: () => rootRoute,
} as any)

const CalcExitRoute = CalcExitImport.update({
  id: '/calc/exit',
  path: '/calc/exit',
  getParentRoute: () => rootRoute,
} as any)

const CalcDistRoute = CalcDistImport.update({
  id: '/calc/dist',
  path: '/calc/dist',
  getParentRoute: () => rootRoute,
} as any)

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/': {
      id: '/'
      path: '/'
      fullPath: '/'
      preLoaderRoute: typeof IndexImport
      parentRoute: typeof rootRoute
    }
    '/about': {
      id: '/about'
      path: '/about'
      fullPath: '/about'
      preLoaderRoute: typeof AboutLazyImport
      parentRoute: typeof rootRoute
    }
    '/calc/dist': {
      id: '/calc/dist'
      path: '/calc/dist'
      fullPath: '/calc/dist'
      preLoaderRoute: typeof CalcDistImport
      parentRoute: typeof rootRoute
    }
    '/calc/exit': {
      id: '/calc/exit'
      path: '/calc/exit'
      fullPath: '/calc/exit'
      preLoaderRoute: typeof CalcExitImport
      parentRoute: typeof rootRoute
    }
    '/calc/fuel': {
      id: '/calc/fuel'
      path: '/calc/fuel'
      fullPath: '/calc/fuel'
      preLoaderRoute: typeof CalcFuelImport
      parentRoute: typeof rootRoute
    }
    '/calc/jump': {
      id: '/calc/jump'
      path: '/calc/jump'
      fullPath: '/calc/jump'
      preLoaderRoute: typeof CalcJumpImport
      parentRoute: typeof rootRoute
    }
    '/calc/path': {
      id: '/calc/path'
      path: '/calc/path'
      fullPath: '/calc/path'
      preLoaderRoute: typeof CalcPathImport
      parentRoute: typeof rootRoute
    }
  }
}

// Create and export the route tree

export interface FileRoutesByFullPath {
  '/': typeof IndexRoute
  '/about': typeof AboutLazyRoute
  '/calc/dist': typeof CalcDistRoute
  '/calc/exit': typeof CalcExitRoute
  '/calc/fuel': typeof CalcFuelRoute
  '/calc/jump': typeof CalcJumpRoute
  '/calc/path': typeof CalcPathRoute
}

export interface FileRoutesByTo {
  '/': typeof IndexRoute
  '/about': typeof AboutLazyRoute
  '/calc/dist': typeof CalcDistRoute
  '/calc/exit': typeof CalcExitRoute
  '/calc/fuel': typeof CalcFuelRoute
  '/calc/jump': typeof CalcJumpRoute
  '/calc/path': typeof CalcPathRoute
}

export interface FileRoutesById {
  __root__: typeof rootRoute
  '/': typeof IndexRoute
  '/about': typeof AboutLazyRoute
  '/calc/dist': typeof CalcDistRoute
  '/calc/exit': typeof CalcExitRoute
  '/calc/fuel': typeof CalcFuelRoute
  '/calc/jump': typeof CalcJumpRoute
  '/calc/path': typeof CalcPathRoute
}

export interface FileRouteTypes {
  fileRoutesByFullPath: FileRoutesByFullPath
  fullPaths:
    | '/'
    | '/about'
    | '/calc/dist'
    | '/calc/exit'
    | '/calc/fuel'
    | '/calc/jump'
    | '/calc/path'
  fileRoutesByTo: FileRoutesByTo
  to:
    | '/'
    | '/about'
    | '/calc/dist'
    | '/calc/exit'
    | '/calc/fuel'
    | '/calc/jump'
    | '/calc/path'
  id:
    | '__root__'
    | '/'
    | '/about'
    | '/calc/dist'
    | '/calc/exit'
    | '/calc/fuel'
    | '/calc/jump'
    | '/calc/path'
  fileRoutesById: FileRoutesById
}

export interface RootRouteChildren {
  IndexRoute: typeof IndexRoute
  AboutLazyRoute: typeof AboutLazyRoute
  CalcDistRoute: typeof CalcDistRoute
  CalcExitRoute: typeof CalcExitRoute
  CalcFuelRoute: typeof CalcFuelRoute
  CalcJumpRoute: typeof CalcJumpRoute
  CalcPathRoute: typeof CalcPathRoute
}

const rootRouteChildren: RootRouteChildren = {
  IndexRoute: IndexRoute,
  AboutLazyRoute: AboutLazyRoute,
  CalcDistRoute: CalcDistRoute,
  CalcExitRoute: CalcExitRoute,
  CalcFuelRoute: CalcFuelRoute,
  CalcJumpRoute: CalcJumpRoute,
  CalcPathRoute: CalcPathRoute,
}

export const routeTree = rootRoute
  ._addFileChildren(rootRouteChildren)
  ._addFileTypes<FileRouteTypes>()

/* ROUTE_MANIFEST_START
{
  "routes": {
    "__root__": {
      "filePath": "__root.tsx",
      "children": [
        "/",
        "/about",
        "/calc/dist",
        "/calc/exit",
        "/calc/fuel",
        "/calc/jump",
        "/calc/path"
      ]
    },
    "/": {
      "filePath": "index.tsx"
    },
    "/about": {
      "filePath": "about.lazy.tsx"
    },
    "/calc/dist": {
      "filePath": "calc/dist.tsx"
    },
    "/calc/exit": {
      "filePath": "calc/exit.tsx"
    },
    "/calc/fuel": {
      "filePath": "calc/fuel.tsx"
    },
    "/calc/jump": {
      "filePath": "calc/jump.tsx"
    },
    "/calc/path": {
      "filePath": "calc/path.tsx"
    }
  }
}
ROUTE_MANIFEST_END */
