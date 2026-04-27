// @webroamer/registry-front-end/vite.config.js
import { defineConfig, loadEnv } from 'vite';

export default defineConfig(({ mode }) => {
  // loading environment variables
  const env = loadEnv(mode, process.cwd(), '');

  return {
    plugins: [],
    server: {
      host: '0.0.0.0',
      port: 6010,
      allowedHosts: ['fpm.localhost'],
      proxy: {
        '/v1': {
          target: env.VITE__DEV_API_PROXY_TARGET || 'http://localhost:3000',
          changeOrigin: true,
          rewrite: (path) => path.replace(/^\/v1/, ''),
        },

        '/docs': {
          target: env.VITE__DEV_API_PROXY_TARGET || 'http://localhost:3000',
          changeOrigin: true,
        },
      },
    },
    build: {
      // generate to dist/assets/build/
      outDir: 'dist',
      rollupOptions: {
        output: {
          entryFileNames: `assets/entry.[hash].js`,
          chunkFileNames: `assets/chunk.[hash].js`,
          assetFileNames: `assets/asset.[hash].[ext]`,
        },
      },
    },
  };
});
