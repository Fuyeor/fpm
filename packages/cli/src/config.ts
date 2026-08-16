// packages/cli/src/config.ts
import { chmod, mkdir, readFile, writeFile } from 'node:fs/promises';
import { homedir, platform } from 'node:os';
import { join } from 'node:path';
import type { FpmConfig } from './types.js';

export const DEFAULT_REGISTRY = 'https://fpm.fuyeor.com/v1';

function normalizeRegistry(registry: string): string {
  const value = registry.trim().replace(/\/+$/, '');
  const url = new URL(value);
  if (url.protocol !== 'https:' && url.protocol !== 'http:')
    throw new Error('Registry URL must use http or https.');
  if (url.username || url.password || url.search || url.hash)
    throw new Error('Registry URL must not contain credentials, query strings, or fragments.');
  return value;
}

export function configPath(): string {
  const home = homedir();
  const base = process.env.XDG_CONFIG_HOME
    ?? (platform() === 'win32'
      ? process.env.APPDATA ?? join(home, 'AppData', 'Local')
      : platform() === 'darwin'
        ? join(home, 'Library', 'Preferences')
        : join(home, '.config'));
  return join(base, 'fpm', 'config.json');
}

export async function readConfig(): Promise<FpmConfig> {
  const file = configPath();
  try {
    const parsed = JSON.parse(await readFile(file, 'utf8')) as Partial<FpmConfig>;
    return {
      registry: normalizeRegistry(parsed.registry ?? DEFAULT_REGISTRY),
      ...(parsed.token ? { token: parsed.token } : {}),
    };
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== 'ENOENT') throw error;
    return { registry: normalizeRegistry(process.env.FPM_REGISTRY ?? DEFAULT_REGISTRY) };
  }
}

export async function resolveConfig(): Promise<FpmConfig> {
  const config = await readConfig();
  const registry = normalizeRegistry(process.env.FPM_REGISTRY ?? config.registry);
  const token = process.env.FPM_TOKEN ?? config.token;
  return token ? { registry, token } : { registry };
}

export async function writeConfig(config: FpmConfig): Promise<void> {
  const file = configPath();
  const directory = join(file, '..');
  await mkdir(directory, { recursive: true, mode: 0o700 });
  await writeFile(file, `${JSON.stringify({ registry: normalizeRegistry(config.registry), ...(config.token ? { token: config.token } : {}) }, null, 2)}\n`, { mode: 0o600 });
  await chmod(directory, 0o700);
  await chmod(file, 0o600);
}

export async function clearToken(): Promise<void> {
  const config = await readConfig();
  await writeConfig({ registry: config.registry });
}
