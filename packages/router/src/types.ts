// @fuyeor/router/src/types.ts
import type { Signal } from 'signal-polyfill';

export type RouteLocationRaw =
  | string
  | {
      name?: string;
      path?: string;
      params?: any;
      query?: any;
      replace?: boolean;
      state?: any;
    };

// In Native Web Components, a component is either a tag name or a module returning a tag name
export type RouteComponent =
  | string
  | (() => Promise<string | { default: string }>);

export interface RouteRecord {
  path: string;
  name?: string;
  redirect?: RouteLocationRaw | ((to: RouteLocation) => RouteLocationRaw);
  component?: RouteComponent;
  children?: RouteRecord[];
  meta?: Record<string, any>;
  props?:
    | boolean
    | Record<string, any>
    | ((route: RouteLocation) => Record<string, any>);
  beforeEnter?: NavigationGuard | NavigationGuard[];
}

export interface RouteMatched {
  name?: string;
  component: string | RouteComponent; // Resolved to tag name at runtime
  meta: Record<string, any>;
  props?: any;
  beforeEnter?: NavigationGuard | NavigationGuard[];
}

export interface RouteLocation {
  fullPath: string;
  path: string;
  name?: string;
  redirect?: RouteLocationRaw | ((to: RouteLocation) => RouteLocationRaw);
  params: Record<string, string>;
  query: Record<string, string>;
  meta: Record<string, any>;
  matched: RouteMatched[];
  isModal: boolean;
}

export interface RouterOptions {
  routes: RouteRecord[];
  linkActiveClass?: string;
  linkExactActiveClass?: string;
}

export type RouteGuardReturn =
  | void
  | boolean
  | string
  | {
      name?: string;
      path?: string;
      query?: any;
      params?: any;
      replace?: boolean;
      state?: any;
    };

export type NavigationGuard = (
  to: RouteLocation,
  from: RouteLocation,
) => RouteGuardReturn | Promise<RouteGuardReturn>;

export interface WebroamerRouter {
  options: RouterOptions;
  currentRoute: Signal.State<RouteLocation>;
  layerStack: Signal.State<RouteLocation[]>;
  push: (to: RouteLocationRaw) => Promise<void>;
  resolve: (to: RouteLocationRaw) => RouteLocation & { href: string };
  replace: (to: RouteLocationRaw) => Promise<void>;
  back: () => Promise<void>;
  beforeEach: (guard: NavigationGuard) => void;
  afterEach: (guard: (to: RouteLocation) => void) => void;
  onError: (handler: (err: any) => void) => void;
}
