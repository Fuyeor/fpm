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

/**
 * Component loader follows native ESM dynamic import pattern.
 * The import should register the custom element as a side effect.
 */
export type RouteComponentLoader = () => Promise<any>;

export interface RouteRecord {
  path: string;
  component?: string; // The custom element component name, e.g., 'pg-home'
  loader?: RouteComponentLoader; // Dynamic import function
  name?: string;
  redirect?: RouteLocationRaw | ((to: RouteLocation) => RouteLocationRaw);
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
  component?: string;
  loader?: RouteComponentLoader;
  meta: Record<string, any>;
  props?: any;
  beforeEnter?: NavigationGuard | NavigationGuard[];
}

export interface RouteLocation {
  fullPath: string;
  path: string;
  name?: string;
  redirect?: RouteRecord['redirect'];
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
  replace: (to: RouteLocationRaw) => Promise<void>;
  resolve: (to: RouteLocationRaw) => RouteLocation & { href: string };
  back: () => Promise<void>;
  beforeEach: (guard: NavigationGuard) => void;
  afterEach: (guard: (to: RouteLocation) => void) => void;
  onError: (handler: (err: any) => void) => void;
}
