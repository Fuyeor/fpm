// @fuyeor/router/src/router.ts
import { Signal } from 'signal-polyfill';
import { createMatcher } from './matcher';
import type {
  RouteRecord,
  WebroamerRouter,
  RouteLocation,
  RouterOptions,
  RouteMatched,
  NavigationGuard,
  RouteLocationRaw,
} from './types';

// Preload web components and resolve to tag names
async function loadComponents(matched: RouteMatched[]) {
  // Parallel loading for all matched route segments
  await window.Promise.all(
    matched.map(async (match) => {
      if (typeof match.component === 'function') {
        const result = await match.component();
        // If it returns a module, we assume it's a side-effect import or returns tag name
        // Pattern: () => import('./view.js').then(m => m.tagName)
        match.component =
          typeof result === 'string' ? result : result?.default || result;

        // Fail-fast: If we still don't have a tag name, the developer forgot to return it
        if (typeof match.component !== 'string') {
          window.console.warn(
            `[Router] Lazy component for "${match.name}" did not return a tag name string.`,
          );
        }
      }
    }),
  );
}

export function createRouter(options: RouterOptions): WebroamerRouter {
  const initialRoute: RouteLocation = {
    path: '/',
    fullPath: '/',
    params: {},
    query: {},
    meta: {},
    matched: [],
    isModal: false,
  };

  const currentRoute = new Signal.State<RouteLocation>(initialRoute);
  const layerStack = new Signal.State<RouteLocation[]>([initialRoute]);

  const guards: NavigationGuard[] = [];
  const afterGuards: Function[] = [];
  const errorHandlers: Function[] = [];

  const updateRouteState = (to: RouteLocation) => {
    const performUpdate = () => {
      currentRoute.set(to);
      const currentStack = layerStack.get();

      if (to.isModal) {
        if (currentStack.length > 1) {
          const newStack = [...currentStack];
          newStack[newStack.length - 1] = to;
          layerStack.set(newStack);
        } else {
          layerStack.set([...currentStack, to]);
        }
      } else {
        layerStack.set([to]);
      }
    };

    // View Transition support
    window.document.startViewTransition(() => performUpdate());
  };

  function resolveUrlString(to: RouteLocationRaw): string {
    if (typeof to === 'string') return to;

    let path = to.path || '';

    if (to.name && !to.path) {
      const targetParams = {
        ...currentRoute.get().params,
        ...(to.params || {}),
      };
      const foundPath = nameMap.get(to.name);

      if (foundPath === undefined)
        throw new window.Error(`[Router] Route Name "${to.name}" not found`);

      path = foundPath
        .replace(/\/\*$/, '')
        .replace(
          /\{\/:([a-zA-Z0-9_]+)(?:\(([^)]+)\))?\}\?/g,
          (_, pName, constraint) => {
            const val = targetParams[pName];
            if (
              val != null &&
              constraint &&
              !new window.RegExp(`^(?:${constraint})$`).test(String(val))
            )
              return '';
            return val != null ? `/${val}` : '';
          },
        )
        .replace(
          /:([a-zA-Z0-9_]+)(?:\(([^)]+)\))?/g,
          (match, pName, constraint) => {
            const val = targetParams[pName];
            if (
              val != null &&
              constraint &&
              !new window.RegExp(`^(?:${constraint})$`).test(String(val))
            )
              return match;
            return val != null ? String(val) : match;
          },
        )
        .replace(/\/+/g, '/');

      path = path || '/';
    }

    path += to.query
      ? `?${window.Object.entries(to.query)
          .map(
            ([k, v]) =>
              `${window.encodeURIComponent(k)}=${window.encodeURIComponent(String(v))}`,
          )
          .join('&')}`
      : '';

    return path;
  }

  const { resolve: resolveMatcher, nameMap } = createMatcher(
    options.routes,
    resolveUrlString,
  );

  const executePipeline = async (toRoute: RouteLocation) => {
    try {
      for (const guard of guards) {
        const res = await guard(toRoute, currentRoute.get());
        if (res === false) throw new window.Error('Navigation Aborted');
        if (typeof res === 'string' || typeof res === 'object')
          return router.replace(res);
      }

      await loadComponents(toRoute.matched);
      updateRouteState(toRoute);
      afterGuards.forEach((guard) => guard(toRoute));
    } catch (err) {
      errorHandlers.forEach((h) => h(err));
    }
  };

  const router: WebroamerRouter = {
    options,
    currentRoute,
    layerStack,
    resolve(to) {
      const href = resolveUrlString(to);
      const matched = resolveMatcher(href);
      if (matched) return { ...matched, href };
      throw new window.Error(`[Router] No route match for ${href}`);
    },
    async push(to) {
      try {
        await window.navigation.navigate(resolveUrlString(to), {
          state: (to as any)?.state,
        }).finished;
      } catch (e: any) {
        if (e.name !== 'AbortError') window.console.error(e);
      }
    },
    async replace(to) {
      try {
        await window.navigation.navigate(resolveUrlString(to), {
          history: 'replace',
          state: (to as any)?.state,
        }).finished;
      } catch (e: any) {
        if (e.name !== 'AbortError') window.console.error(e);
      }
    },
    async back() {
      try {
        await window.navigation.back().finished;
      } catch (e: any) {
        if (e.name !== 'AbortError') window.console.error(e);
      }
    },
    beforeEach: (guard) => guards.push(guard),
    afterEach: (guard) => afterGuards.push(guard),
    onError: (handler) => errorHandlers.push(handler),
  };

  // Bind Native Navigation API seamlessly
  window.navigation.addEventListener('navigate', (event: any) => {
    const url = new window.URL(event.destination.url);
    const toRoute = resolveMatcher(url.pathname + url.search);

    if (toRoute) {
      event.intercept({
        handler: () => {
          executePipeline(toRoute);
        },
      });
    }
  });

  // Initial Boot
  window.Promise.resolve().then(() => {
    const initRoute = resolveMatcher(
      window.location.pathname + window.location.search,
    );
    if (initRoute) executePipeline(initRoute);
  });

  return router;
}
