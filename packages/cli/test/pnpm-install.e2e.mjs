// packages/cli/test/pnpm-install.e2e.mjs
import { createServer } from 'node:http';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { packPackage } from '../dist/pack.js';

const execFileAsync = promisify(execFile);
const cleanup = [];

async function createConsumer(registry, dependency) {
  const root = await mkdtemp(join(tmpdir(), 'fpm-e2e-consumer-'));
  cleanup.push(root);
  await writeFile(join(root, 'package.json'), JSON.stringify({ name: 'consumer-fixture', version: '1.0.0', private: true, ...(dependency ? { dependencies: { '@demo/install-fixture': dependency } } : {}) }));
  await writeFile(join(root, '.npmrc'), `@demo:registry=${registry}\n`);
  return root;
}

async function assertInstalled(root) {
  const installed = await readFile(join(root, 'node_modules/@demo/install-fixture/index.js'), 'utf8');
  if (!installed.includes('installed: true')) throw new Error('Installed package content did not match the published tarball.');
}

try {
  const packageRoot = await mkdtemp(join(tmpdir(), 'fpm-e2e-package-'));
  cleanup.push(packageRoot);
  await writeFile(join(packageRoot, 'package.json'), JSON.stringify({ name: '@demo/install-fixture', version: '1.0.0', main: 'index.js' }));
  await writeFile(join(packageRoot, 'index.js'), 'module.exports = { installed: true };\n');
  const packed = await packPackage(packageRoot);
  const integrity = `sha256-${Buffer.from(packed.sha256, 'hex').toString('base64')}`;

  const server = createServer(async (request, response) => {
    const url = new URL(request.url ?? '/', 'http://127.0.0.1');
    if (url.pathname === '/v1/@demo%2Finstall-fixture' || decodeURIComponent(url.pathname) === '/v1/@demo/install-fixture') {
      response.setHeader('Content-Type', 'application/json');
      response.end(JSON.stringify({
        name: '@demo/install-fixture',
        'dist-tags': { latest: '1.0.0' },
        versions: {
          '1.0.0': {
            name: '@demo/install-fixture',
            version: '1.0.0',
            dist: {
              tarball: `http://127.0.0.1:${server.address().port}/packages/@demo/install-fixture/1.0.0.tgz`,
              shasum: packed.sha256,
              integrity,
            },
          },
        },
      }));
      return;
    }
    if (url.pathname === '/packages/@demo/install-fixture/1.0.0.tgz') {
      response.setHeader('Content-Type', 'application/gzip');
      response.end(Buffer.from(packed.tarball));
      return;
    }
    response.statusCode = 404;
    response.end('not found');
  });
  await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
  const registry = `http://127.0.0.1:${server.address().port}/v1`;

  const latestConsumer = await createConsumer(registry);
  await execFileAsync('npx', ['--yes', 'pnpm@11.22.0', 'add', '@demo/install-fixture'], { cwd: latestConsumer, timeout: 120_000 });
  await assertInstalled(latestConsumer);
  await execFileAsync('npx', ['--yes', 'pnpm@11.22.0', 'install', '--ignore-scripts'], { cwd: latestConsumer, timeout: 120_000 });
  await assertInstalled(latestConsumer);

  const exactConsumer = await createConsumer(registry);
  await execFileAsync('npx', ['--yes', 'pnpm@11.22.0', 'add', '@demo/install-fixture@1.0.0'], { cwd: exactConsumer, timeout: 120_000 });
  await assertInstalled(exactConsumer);

  server.close();
  process.stdout.write('pnpm add latest, pnpm install, and pnpm add exact version e2e passed\n');
} finally {
  await Promise.all(cleanup.map((directory) => rm(directory, { recursive: true, force: true })));
}
