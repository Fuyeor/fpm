// @/router/index.ts
import {
  createRouter,
  RouteLocation,
  RouterView,
  type RouteRecord,
} from '@fuyeor/vue-router';
import { useTransitionBar } from '@fuyeor/interactify';
import { useLocaleStore } from '@fuyeor/commons';
import { useUserStore } from '@/stores/user';
import { LOCALE_REGEX } from '@/config/locales';

const { start, done } = useTransitionBar();

const appRoutes: Array<RouteRecord> = [
  {
    // fpm.fuyeor.com/auth/callback
    path: 'auth/callback',
    name: 'AuthCallback',
    component: () => import('@/views/AuthCallback.vue'),
    meta: {
      public: true,
      titleKey: 'signin',
    },
  },
  {
    // fpm.fuyeor.com/
    path: '{/:tab}?',
    name: 'Home',
    component: () => import('@/views/Home.vue'),
    meta: {
      public: true,
      overrideTitle: ['site.name', ':', 'site.title'],
    },
  },
  {
    // fpm.fuyeor.com/@Fuyeor
    path: '@:username{/:tab(organizations|packages)}?',
    name: 'Profile',
    component: () => import('@/views/Profile.vue'),
    props: true,
    meta: {
      public: true,
      areaKey: 'profile',
      title: (route: RouteLocation) => `@${String(route.params.username)}`,
    },
  },
  {
    // fpm.fuyeor.com/organization/@fuyeor
    path: 'organization/@:username{/:tab(members|packages)}?',
    name: 'Organization',
    component: () => import('@/views/Profile.vue'),
    props: true,
    meta: {
      public: true,
      areaKey: 'profile',
    },
  },
  {
    // fpm.fuyeor.com/organization/@fuyeor
    path: 'organization/@:username{/:tab(members|packages)}?',
    name: 'Organization',
    component: () => import('@/views/Profile.vue'),
    props: true,
    meta: {
      public: true,
      areaKey: 'profile',
    },
  },
  {
    // fpm.fuyeor.com/package/@fuyeor/vue-router
    path: 'package/:packageName{/:tab(versions|dependencies|dependents)}?',
    name: 'Package',
    component: () => import('@/views/Profile.vue'),
    props: true,
    meta: {
      public: true,
      areaKey: 'package',
    },
  },
  {
    // fpm.fuyeor.com/search
    path: 'search',
    name: 'Search',
    component: () => import('@/views/Profile.vue'),
    props: true,
    meta: {
      public: true,
      areaKey: 'search',
    },
  },
  // fpm.fuyeor.com/options
  {
    path: 'options',
    name: 'Option',
    component: () => import('@/views/Options/Index.vue'), // 作为布局/容器组件
    meta: {
      areaKey: 'settings',
      titleKey: 'settings',
    },
    children: [
      {
        // fpm.fuyeor.com/options/account
        path: 'account',
        name: 'Option.Account',
      },
      {
        // fpm.fuyeor.com/options/tokens
        path: 'tokens',
        name: 'Option.Token',
        component: () => import('@/views/Options/Tokens.vue'),
      },
    ],
  },
];

// root router
const routes: Array<RouteRecord> = [
  {
    // optional locale prefix wrapper, only allows supported locales
    path: `{/:locale(${LOCALE_REGEX})}?`,
    component: RouterView,
    // 将所有应用路由放入 children
    // 注意：子路由的 path 如果不以 / 开头，会拼接在父路由后面
    // 例如：/en/thought/123 或 /thought/123
    children: appRoutes,
  },

  // 404 路由必须放在最外层，且在最后
  // 如果 URL 是 /xx/thought/123 (不支持的语言)，正则匹配失败，会落到这里
  {
    path: '/*', // 通配符路由，捕获所有未匹配的路径
    name: 'NotFound',
    component: () =>
      import('@fuyeor/interactify/views').then((m) => m.NotFoundView),
    meta: {
      public: true,
      titleKey: 'notFound.title',
    },
  },
];

// 创建路由实例
const router = createRouter({ routes });

// 路由守卫
router.beforeEach(async (to, from) => {
  // 启动顶部进度条
  start();

  const userStore = useUserStore();
  // 获取语言 store
  const localeStore = useLocaleStore();

  // 从路由参数中获取 locale (例如 'en' 或 undefined)
  const routeLocale = to.params.locale as string | undefined;

  // 如果路由中有语言参数，且与当前 store 中的不一致，强制切换
  // 这会触发 initializeLocale 中的 loadLocaleMessages，加载新语言包
  if (routeLocale && routeLocale !== localeStore.locale) {
    try {
      await localeStore.setLocale(routeLocale);
    } catch (e) {
      console.error('[RouterGuard] Failed to load locale:', e);
      // 加载语言失败不应该阻塞跳转，只不过界面可能显示默认语言
    }
  }

  // 如果用户已认证但信息未加载，则加载信息。
  // 这是整个守卫中唯一需要加载数据的地方。
  if (userStore.isAuthenticated && !userStore.isUserLoaded) {
    try {
      await userStore.loadCurrentSession();
    } catch {
      // 如果加载失败，store 内部的 onSignOut 会处理状态，
      // authStore.isAuthenticated 将变为 false，后续逻辑会自动处理。
      console.warn(
        '[RouterGuard] User info load failed, session is now invalid.',
      );
    }
  }

  // 检查路由是否需要认证
  const requiresAuth = to.meta.public !== true;

  if (requiresAuth && !userStore.isAuthenticated) {
    // 存业务跳转目标
    window.sessionStorage.setItem('redirect_target', to.fullPath);

    // 如果需要认证但用户未认证，重定向到登录页
    // 保持当前的 locale 参数，如果用户在 /en/ 登录，登录后应该还在 /en/
    return {
      name: 'Welcome',
      params: to.params, // 传递 params 以保留 locale
      state: {
        showAuthHint: true,
      },
    };
  }

  // 如果代码能执行到这里，说明所有检查都通过了，允许导航
  return true;
});

router.afterEach(() => {
  done();
});

router.onError(() => {
  done();
});

export default router;
