// packages/cli/test/pack.test.ts
import { mkdtemp, mkdir, readFile, writeFile } from 'node:fs/promises';
import { gunzipSync } from 'node:zlib';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import tar from 'tar-stream';
import { afterEach, describe, expect, it } from 'vitest';
import { packPackage } from '../src/pack.js';

const temporaryDirectories: string[] = [];

afterEach(async () => {
  const { rm } = await import('node:fs/promises');
  await Promise.all(temporaryDirectories.splice(0).map((directory) => rm(directory, { recursive: true, force: true })));
});

describe('packPackage', () => {
  it('rewrites workspace dependencies to published SemVer ranges', async () => {
    const workspaceRoot = await mkdtemp(join(tmpdir(), 'fpm-workspace-'));
    temporaryDirectories.push(workspaceRoot);
    await mkdir(join(workspaceRoot, 'packages', 'dependency'), { recursive: true });
    await mkdir(join(workspaceRoot, 'packages', 'consumer'), { recursive: true });
    await writeFile(join(workspaceRoot, 'pnpm-workspace.yaml'), 'packages:\n  - packages/*\n');
    await writeFile(join(workspaceRoot, 'packages', 'dependency', 'package.json'), JSON.stringify({ name: '@demo/dependency', version: '2.3.4' }));
    await writeFile(join(workspaceRoot, 'packages', 'consumer', 'package.json'), JSON.stringify({ name: '@demo/consumer', version: '1.0.0', dependencies: { '@demo/dependency': 'workspace:*' } }));
    await writeFile(join(workspaceRoot, 'packages', 'consumer', 'index.js'), 'export {};\n');

    const packed = await packPackage(join(workspaceRoot, 'packages', 'consumer'));
    const extracted = new Map<string, Buffer>();
    const extract = tar.extract();
    extract.on('entry', (header, stream, next) => {
      const chunks: Buffer[] = [];
      stream.on('data', (chunk: Buffer) => chunks.push(chunk));
      stream.on('end', () => {
        extracted.set(header.name, Buffer.concat(chunks));
        next();
      });
      stream.resume();
    });
    const end = new Promise<void>((resolve, reject) => {
      extract.on('finish', resolve);
      extract.on('error', reject);
    });
    extract.end(gunzipSync(packed.tarball));
    await end;
    const publishedManifest = JSON.parse((extracted.get('package/package.json') ?? Buffer.from('{}')).toString()) as { dependencies?: Record<string, string> };
    expect(publishedManifest.dependencies?.['@demo/dependency']).toBe('=2.3.4');
  });

  it('publishes source files selected by files and .gitignore', async () => {
    const directory = await mkdtemp(join(tmpdir(), 'fpm-pack-'));
    temporaryDirectories.push(directory);
    await mkdir(join(directory, 'src'), { recursive: true });
    await mkdir(join(directory, 'node_modules', 'ignored'), { recursive: true });
    await writeFile(join(directory, 'package.json'), JSON.stringify({ name: '@demo/source-package', version: '1.2.3', files: ['src'] }));
    await writeFile(join(directory, '.gitignore'), 'ignored.ts\n');
    await writeFile(join(directory, 'src', 'index.ts'), 'export const answer = 42;\n');
    await writeFile(join(directory, 'src', 'ignored.ts'), 'ignored\n');
    await writeFile(join(directory, 'node_modules', 'ignored', 'index.js'), 'ignored\n');

    const packed = await packPackage(directory);
    expect(packed.files).toEqual(['package.json', 'src/index.ts']);
    expect(packed.sha256).toMatch(/^[0-9a-f]{64}$/);

    const extracted = new Map<string, Buffer>();
    const extract = tar.extract();
    extract.on('entry', (header, stream, next) => {
      const chunks: Buffer[] = [];
      stream.on('data', (chunk: Buffer) => chunks.push(chunk));
      stream.on('end', () => {
        extracted.set(header.name, Buffer.concat(chunks));
        next();
      });
      stream.resume();
    });
    const end = new Promise<void>((resolve, reject) => {
      extract.on('finish', resolve);
      extract.on('error', reject);
    });
    extract.end(gunzipSync(packed.tarball));
    await end;
    expect([...extracted.keys()]).toEqual(['package/package.json', 'package/src/index.ts']);
    expect(JSON.parse((extracted.get('package/package.json') ?? Buffer.from('{}')).toString()).name).toBe('@demo/source-package');
    expect((extracted.get('package/src/index.ts') ?? Buffer.from('')).toString()).toContain('answer');
    expect(await readFile(join(directory, 'src', 'index.ts'), 'utf8')).toContain('answer');
  });

  it('includes explicitly selected ignored build files', async () => {
    const directory = await mkdtemp(join(tmpdir(), 'fpm-pack-dist-'));
    temporaryDirectories.push(directory);
    await mkdir(join(directory, 'dist'), { recursive: true });
    await writeFile(join(directory, 'package.json'), JSON.stringify({ name: '@demo/dist-package', version: '1.0.0', files: ['dist'] }));
    await writeFile(join(directory, '.gitignore'), 'dist\n');
    await writeFile(join(directory, 'dist', 'index.js'), 'export const ready = true;\n');

    const packed = await packPackage(directory);

    expect(packed.files).toEqual(expect.arrayContaining(['package.json', 'dist/index.js']));
    expect(packed.files).toHaveLength(2);
  });
});
