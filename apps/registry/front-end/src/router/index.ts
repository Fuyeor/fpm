// src/router/index.ts
import { initRouter } from '@fuyeor/router';

export const router = initRouter({
  routes: [
    {
      // fpm.fuyeor.com
      path: '/',
      name: 'welcome',
      component: () => import('../views/welcome').then((m) => 'fpm-welcome'),
    },
    {
      // 404
      path: '*',
      name: 'not-found',
      component: () => import('../views/not-found').then((m) => 'not-found'),
    },
  ],
});
