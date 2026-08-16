// packages/cli/test/publish.e2e.mjs
import { createHash } from 'node:crypto';
import { createServer } from 'node:http';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const execFileAsync = promisify(execFile);
const cleanup = [];
const requests = [];
const repositoryRoot = fileURLToPath(new URL('../../..', import.meta.url));

function readBody(request) {
  return new Promise((resolve, reject) => {
    const chunks = [];
    request.on('data', (chunk) => chunks.push(chunk));
    request.on('end', () => resolve(Buffer.concat(chunks)));
    request.on('error', reject);
  });
}

try {
  const packageRoot = await mkdtemp(join(tmpdir(), 'fpm-publish-package-'));
  const configRoot = await mkdtemp(join(tmpdir(), 'fpm-publish-config-'));
  cleanup.push(packageRoot, configRoot);
  await writeFile(join(packageRoot, 'package.json'), JSON.stringify({ name: '@demo/publish-fixture', version: '1.0.0', main: 'index.js' }));
  await writeFile(join(packageRoot, 'index.js'), 'export const published = true;\n');

  const server = createServer(async (request, response) => {
    const body = await readBody(request);
    requests.push({ method: request.method, url: request.url, authorization: request.headers.authorization, body });
    if (request.method === 'POST' && request.url === '/v1/packages/acquire') {
      response.setHeader('Content-Type', 'application/json');
      response.end(JSON.stringify({ uploadUrl: `http://127.0.0.1:${server.address().port}/upload`, uploadSessionId: 'session-1' }));
      return;
    }
    if (request.method === 'PUT' && request.url === '/upload') {
      response.statusCode = 200;
      response.end();
      return;
    }
    if (request.method === 'POST' && request.url === '/v1/packages/commit') {
      response.statusCode = 201;
      response.end();
      return;
    }
    response.statusCode = 404;
    response.end('not found');
  });
  await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
  const registry = `http://127.0.0.1:${server.address().port}/v1`;
  const { stdout } = await execFileAsync(process.execPath, ['packages/cli/dist/cli.js', 'publish', packageRoot, '--json'], {
    cwd: repositoryRoot,
    env: { ...process.env, FPM_REGISTRY: registry, FPM_TOKEN: 'fpm_test-token', XDG_CONFIG_HOME: configRoot },
    timeout: 120_000,
  });
  const result = JSON.parse(stdout);
  if (result.name !== '@demo/publish-fixture' || result.version !== '1.0.0') throw new Error('CLI returned an unexpected publish result.');
  if (requests.length !== 3) throw new Error(`Expected 3 requests, got ${requests.length}.`);
  if (requests[0].authorization !== 'Bearer fpm_test-token' || requests[2].authorization !== 'Bearer fpm_test-token') throw new Error('CLI did not send the PAT Bearer token.');
  const acquire = JSON.parse(requests[0].body.toString());
  const uploadedHash = createHash('sha256').update(requests[1].body).digest('hex');
  if (uploadedHash !== acquire.shasum) throw new Error('CLI sent a checksum that does not match the uploaded bytes.');
  const commit = JSON.parse(requests[2].body.toString());
  if (commit.uploadSessionId !== 'session-1' || commit.manifest.name !== '@demo/publish-fixture') throw new Error('CLI commit payload was incorrect.');
  server.close();
  process.stdout.write('fpm publish e2e passed\n');
} finally {
  await Promise.all(cleanup.map((directory) => rm(directory, { recursive: true, force: true })));
}
