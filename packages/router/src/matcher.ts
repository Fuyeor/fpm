// @fuyeor/router/src/matcher.ts
import type {
  RouteRecord,
  RouteLocation,
  RouteLocationRaw,
  RouteMatched,
} from './types';

interface CompiledRoute {
  pattern: URLPattern;
  record: RouteRecord;
  matchedList: RouteMatched[];
  priority: number;
}

type UrlResolver = (to: RouteLocationRaw) => string;

const normalizePath = (parentPath: string, path: string) => {
  const sep =
    path.startsWith('/') || path.startsWith('{/') || !parentPath ? '' : '/';
  let joined = `${parentPath}${sep}${path}`.replace(/\/+/g, '/');

  if (!parentPath && !joined.startsWith('/') && !joined.startsWith('{/'))
    joined = `/${joined}`;

  const pattern = joined === '/' ? '' : joined.replace(/\/$/, '');
  const isWildcard = pattern.replace(/\([^)]+\)/g, '').includes('*');

  const priority = isWildcard
    ? -1
    : pattern
        .split('/')
        .reduce(
          (acc, seg) =>
            acc + (seg.includes('(') ? 2 : seg.includes(':') ? 1 : seg ? 4 : 0),
          0,
        );

  return { pattern, priority };
};

export function createMatcher(routes: RouteRecord[], urlResolver: UrlResolver) {
  const compiledRoutes: CompiledRoute[] = [];
  const nameMap = new Map<string, string>();

  function buildRoute(
    records: RouteRecord[],
    parentPath = '',
    matchedParent: RouteMatched[] = [],
  ) {
    for (const record of records) {
      const { pattern, priority } = normalizePath(parentPath, record.path);
      if (record.name) nameMap.set(record.name, pattern);

      const matchedList = [
        ...matchedParent,
        {
          name: record.name,
          component: record.component!,
          meta: record.meta || {},
          props: record.props,
          beforeEnter: record.beforeEnter,
        },
      ];

      if (record.name || record.component || record.redirect) {
        try {
          compiledRoutes.push({
            pattern: new window.URLPattern({ pathname: pattern }),
            record,
            matchedList,
            priority,
          });
        } catch {
          throw new window.Error(
            `[Router] URLPattern build failed: ${pattern}`,
          );
        }
      }

      if (record.children) buildRoute(record.children, pattern, matchedList);
    }
  }

  buildRoute(routes);
  compiledRoutes.sort((a, b) => b.priority - a.priority);

  const resolve = (url: string): RouteLocation | null => {
    const [pathPart, queryPart] = url.split('?');
    let searchPath = pathPart || '';

    if (searchPath.endsWith('/')) searchPath = searchPath.slice(0, -1);

    for (const { pattern, record, matchedList } of compiledRoutes) {
      const match = pattern.exec({ pathname: searchPath });
      if (match) {
        const params: Record<string, string> = {};
        for (const [key, val] of window.Object.entries(match.pathname.groups)) {
          if (val !== undefined) params[key] = val;
        }

        const resolvedRoute: RouteLocation = {
          fullPath: url,
          path: pathPart || '/',
          name: record.name,
          params,
          query: window.Object.fromEntries(
            new window.URLSearchParams(queryPart || '').entries(),
          ),
          meta: record.meta || {},
          matched: matchedList,
          isModal: !!record.meta?.isModal,
          redirect: record.redirect,
        };

        if (resolvedRoute.redirect) {
          const target =
            typeof resolvedRoute.redirect === 'function'
              ? resolvedRoute.redirect(resolvedRoute)
              : resolvedRoute.redirect;
          return resolve(urlResolver(target));
        }
        return resolvedRoute;
      }
    }
    return null;
  };

  return { resolve, nameMap };
}
