// @fuyeor/router/src/index.ts
import { createRouter } from './router';
import type { RouterOptions } from './types';

import './components/router-link';
import './components/router-view';

export let router: ReturnType<typeof createRouter>;

// App initialization function
export function initRouter(options: RouterOptions) {
  if (router) throw new window.Error('Router already initialized!');
  router = createRouter(options);
  return router;
}

// gets the route from properties if deep inside modal,
// or falls back to global currentRoute
export function useRoute(elementContext?: HTMLElement) {
  // If element was rendered by router-view, it gets a direct private route object
  if (elementContext && (elementContext as any).routeContext) {
    return (elementContext as any).routeContext;
  }
  return router.currentRoute.get();
}

export function useRouter() {
  return router;
}

export * from './types';
