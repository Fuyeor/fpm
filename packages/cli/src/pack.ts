// packages/cli/src/pack.ts
import { gzipSync } from 'node:zlib';
import { createHash } from 'node:crypto';
import { lstat, readFile, readdir } from 'node:fs/promises';
import { dirname, join, resolve, sep } from 'node:path';
import ignore from 'ignore';
import tar from 'tar-stream';
import semver from 'semver';
import type { PackageManifest } from './types.js';

export interface PackedPackage {
  manifest: PackageManifest;
  tarball: Uint8Array;
  sha256: string;
  files: string[];
}

const WORKSPACE_DEPENDENCY_FIELDS = [
  'dependencies',
  'devDependencies',
  'optionalDependencies',
  'peerDependencies',
] as const;

type DependencyField = (typeof WORKSPACE_DEPENDENCY_FIELDS)[number];

function normalizePath(filePath: string): string {
  return filePath.split(sep).join('/').replace(/^\.\//, '');
}

function globToRegExp(pattern: string): RegExp {
  let source = '^';
  for (let index = 0; index < pattern.length; index += 1) {
    const character = pattern[index];
    if (character === undefined) continue;
    if (character === '*') {
      if (pattern[index + 1] === '*') {
        source += '.*';
        index += 1;
      } else {
        source += '[^/]*';
      }
    } else if (character === '?') {
      source += '[^/]';
    } else {
      source += /[\\^$+?.()|[\]{}]/.test(character) ? `\\${character}` : character;
    }
  }
  return new RegExp(`${source}(?:/.*)?$`);
}

function matchesExplicitFiles(filePath: string, files: string[]): boolean {
  return files.some((entry) => {
    const pattern = normalizePath(entry).replace(/\/$/, '');
    if (!pattern) return false;
    if (!pattern.includes('*') && !pattern.includes('?')) return filePath === pattern || filePath.startsWith(`${pattern}/`);
    return globToRegExp(pattern).test(filePath);
  });
}

async function collectFiles(root: string, relativeDirectory = ''): Promise<string[]> {
  const directory = join(root, relativeDirectory);
  const entries = await readdir(directory, { withFileTypes: true });
  const files: string[] = [];
  for (const entry of entries) {
    const relativePath = normalizePath(join(relativeDirectory, entry.name));
    if (entry.isDirectory()) {
      if (entry.name === 'node_modules' || entry.name === '.git') continue;
      files.push(...await collectFiles(root, relativePath));
    } else {
      files.push(relativePath);
    }
  }
  return files;
}

async function findWorkspaceRoot(directory: string): Promise<string | undefined> {
  let current = resolve(directory);
  while (true) {
    try {
      await readFile(join(current, 'pnpm-workspace.yaml'), 'utf8');
      return current;
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== 'ENOENT') throw error;
    }
    const parent = dirname(current);
    if (parent === current) return undefined;
    current = parent;
  }
}

async function collectWorkspaceVersions(root: string, relativeDirectory = ''): Promise<Map<string, string>> {
  const directory = join(root, relativeDirectory);
  const entries = await readdir(directory, { withFileTypes: true });
  const versions = new Map<string, string>();
  for (const entry of entries) {
    if (entry.name === 'node_modules' || entry.name === '.git' || entry.name === 'target') continue;
    const relativePath = normalizePath(join(relativeDirectory, entry.name));
    if (entry.isDirectory()) {
      for (const [name, version] of await collectWorkspaceVersions(root, relativePath)) versions.set(name, version);
      continue;
    }
    if (entry.name !== 'package.json') continue;
    const manifest = JSON.parse(await readFile(join(root, relativePath), 'utf8')) as Partial<PackageManifest>;
    if (typeof manifest.name === 'string' && typeof manifest.version === 'string') versions.set(manifest.name, manifest.version);
  }
  return versions;
}

function rewriteWorkspaceRange(range: string, packageName: string, versions: Map<string, string>): string {
  if (!range.startsWith('workspace:')) return range;
  const actualVersion = versions.get(packageName);
  if (!actualVersion) throw new Error(`Workspace dependency ${packageName} is not available in the workspace.`);
  const requested = range.slice('workspace:'.length);
  if (requested === '*' || requested === '') return `=${actualVersion}`;
  if (requested === '^' || requested === '~') return `${requested}${actualVersion}`;
  return requested;
}

async function normalizePublishedManifest(root: string, manifest: PackageManifest): Promise<PackageManifest> {
  const workspaceRoot = await findWorkspaceRoot(root);
  if (!workspaceRoot) return manifest;
  const versions = await collectWorkspaceVersions(workspaceRoot);
  const result: PackageManifest = { ...manifest };
  for (const field of WORKSPACE_DEPENDENCY_FIELDS) {
    const dependencies = manifest[field] as Record<string, unknown> | undefined;
    if (!dependencies || typeof dependencies !== 'object' || Array.isArray(dependencies)) continue;
    result[field] = Object.fromEntries(Object.entries(dependencies).map(([name, range]) => [
      name,
      typeof range === 'string' ? rewriteWorkspaceRange(range, name, versions) : range,
    ]));
  }
  return result;
}

function validateManifest(manifest: PackageManifest): void {
  if (!manifest.name || !manifest.name.startsWith('@') || !manifest.name.includes('/'))
    throw new Error('package.json name must be a scoped package such as @scope/name.');
  if (!semver.valid(manifest.version)) throw new Error(`Invalid SemVer package version: ${manifest.version}`);
}

export async function readManifest(directory: string): Promise<PackageManifest> {
  const manifestPath = join(directory, 'package.json');
  const manifest = JSON.parse(await readFile(manifestPath, 'utf8')) as PackageManifest;
  validateManifest(manifest);
  return manifest;
}

export async function packPackage(directory: string): Promise<PackedPackage> {
  const root = resolve(directory);
  const sourceManifest = await readManifest(root);
  const manifest = await normalizePublishedManifest(root, sourceManifest);
  const gitignore = ignore();
  try {
    gitignore.add(await readFile(join(root, '.gitignore'), 'utf8'));
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== 'ENOENT') throw error;
  }

  const declaredFiles = manifest.files?.filter((entry): entry is string => typeof entry === 'string');
  const candidates = await collectFiles(root);
  const files = candidates.filter((filePath) => {
    if (filePath === '.gitignore' || filePath.startsWith('.git/') || filePath.startsWith('node_modules/')) return false;
    if (gitignore.ignores(filePath)) return false;
    if (declaredFiles && declaredFiles.length > 0 && filePath !== 'package.json' && !matchesExplicitFiles(filePath, declaredFiles)) return false;
    return true;
  });
  if (!files.includes('package.json')) files.unshift('package.json');
  if (files.length === 0) throw new Error('Package has no files to publish.');

  const pack = tar.pack();
  const chunks: Buffer[] = [];
  const output = new Promise<Buffer>((resolveOutput, rejectOutput) => {
    pack.on('data', (chunk: Buffer) => chunks.push(chunk));
    pack.on('end', () => resolveOutput(Buffer.concat(chunks)));
    pack.on('error', rejectOutput);
  });

  for (const filePath of files.sort()) {
    const absolutePath = join(root, filePath);
    const stats = await lstat(absolutePath);
    if (!stats.isFile()) throw new Error(`Only regular files can be published: ${filePath}`);
    const content = filePath === 'package.json'
      ? Buffer.from(`${JSON.stringify(manifest, null, 2)}\n`)
      : await readFile(absolutePath);
    pack.entry({
      name: `package/${filePath}`,
      size: content.byteLength,
      mode: stats.mode & 0o111 ? 0o755 : 0o644,
      mtime: new Date(0),
    }, content);
  }
  pack.finalize();

  const tarball = gzipSync(await output, { level: 9 });
  const sha256 = createHash('sha256').update(tarball).digest('hex');
  return { manifest, tarball, sha256, files };
}
