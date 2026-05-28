// @fuyeor/fpm-front-end/vite.config.js
import { defineConfig, loadEnv } from 'vite';
import { createViteConfig } from '@fuyeor/config/vite.config.js';

export default defineConfig(({ mode }) => {
  // loading environment variables
  const env = loadEnv(mode, process.cwd(), '');

  // define proxy target for development
  const apiTarget = env.VITE__DEV_API_PROXY_TARGET || 'http://localhost:3000';

  return createViteConfig(
    {
      server: {
        host: '0.0.0.0',
        port: 6010,
        allowedHosts: ['fpm.localhost'],
        proxy: {
          '/v1': {
            target: apiTarget,
            changeOrigin: true,
            rewrite: (path) => path.replace(/^\/v1/, ''),
          },
          '/docs': {
            target: apiTarget,
            changeOrigin: true,
          },
        },
      },
    },
    import.meta.dirname,
  );
});
