// src/router/index.ts
import { initRouter } from '@fuyeor/router';

export const router = initRouter({
  routes: [
    {
      // fpm.fuyeor.com/welcome
      path: '/welcome',
      name: 'Welcome',
      component: 'fpm-welcome',
      loader: () => import('../views/welcome'),
    },
    {
      // fpm.fuyeor.com/
      path: '/',
      name: 'Home',
      component: 'fpm-home',
      loader: () => import('../views/home'),
    },
    {
      // 404
      path: '*',
      name: 'NotFound',
      component: 'not-found',
      loader: () => import('../views/not-found'),
    },
  ],
});
