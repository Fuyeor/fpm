// packages/cli/src/commands.ts
import { createInterface } from 'node:readline/promises';
import { stdin, stdout } from 'node:process';
import { acquireUpload, commitUpload, uploadTarball } from './api.js';
import { clearToken, DEFAULT_REGISTRY, readConfig, resolveConfig, writeConfig } from './config.js';
import { packPackage } from './pack.js';
import type { FpmConfig } from './types.js';

async function readSecret(prompt: string): Promise<string> {
  if (!stdin.isTTY || !stdin.setRawMode) {
    const readline = createInterface({ input: stdin, output: stdout });
    const value = await readline.question(prompt);
    readline.close();
    return value.trim();
  }

  stdout.write(prompt);
  stdin.setRawMode(true);
  stdin.resume();
  stdin.setEncoding('utf8');
  return await new Promise<string>((resolve, reject) => {
    let value = '';
    const onData = (chunk: string) => {
      for (const character of chunk) {
        if (character === '\u0003') {
          stdin.setRawMode(false);
          stdin.pause();
          stdin.off('data', onData);
          reject(new Error('Input cancelled.'));
          return;
        }
        if (character === '\r' || character === '\n') {
          stdout.write('\n');
          stdin.setRawMode(false);
          stdin.pause();
          stdin.off('data', onData);
          resolve(value.trim());
          return;
        }
        if (character === '\u007f') {
          value = value.slice(0, -1);
          continue;
        }
        value += character;
      }
    };
    stdin.on('data', onData);
  });
}

function requireToken(config: FpmConfig): FpmConfig & { token: string } {
  if (!config.token) throw new Error('No FPM token configured. Run `fpm login` or set FPM_TOKEN.');
  return config as FpmConfig & { token: string };
}

export async function loginCommand(registry = process.env.FPM_REGISTRY ?? DEFAULT_REGISTRY): Promise<void> {
  const token = await readSecret('FPM token: ');
  if (!token.startsWith('fpm_')) throw new Error('Invalid FPM token format.');
  await writeConfig({ registry, token });
  stdout.write(`Saved credentials for ${registry.replace(/\/+$/, '')}.\n`);
}

export async function logoutCommand(): Promise<void> {
  await clearToken();
  stdout.write('Removed the saved FPM token.\n');
}

export async function whoamiCommand(): Promise<void> {
  const config = requireToken(await resolveConfig());
  const { whoami } = await import('./api.js');
  const user = await whoami(config);
  stdout.write(`${user.username}\n`);
}

export async function publishCommand(directory: string, options: { dryRun: boolean; json: boolean; registry?: string | undefined }): Promise<void> {
  const packed = await packPackage(directory);
  if (options.dryRun) {
    if (options.json) stdout.write(`${JSON.stringify({ name: packed.manifest.name, version: packed.manifest.version, files: packed.files, sha256: packed.sha256 })}\n`);
    else stdout.write(`Would publish ${packed.manifest.name}@${packed.manifest.version} (${packed.files.length} files, ${packed.tarball.byteLength} bytes)\n`);
    return;
  }

  const resolved = await resolveConfig();
  const config = requireToken(options.registry ? { ...resolved, registry: options.registry } : resolved);
  const acquired = await acquireUpload(config, packed.manifest, packed.sha256);
  await uploadTarball(acquired.uploadUrl, packed.tarball);
  await commitUpload(config, acquired.uploadSessionId, packed.manifest);
  const result = {
    name: packed.manifest.name,
    version: packed.manifest.version,
    tarball: `${config.registry}/packages/${packed.manifest.name}/${packed.manifest.version}.tgz`,
    sha256: packed.sha256,
  };
  if (options.json) stdout.write(`${JSON.stringify(result)}\n`);
  else stdout.write(`Published ${result.name}@${result.version}\nTarball: ${result.tarball}\nSHA-256: ${result.sha256}\n`);
}

export async function configCommand(): Promise<void> {
  const config = await readConfig();
  stdout.write(`${JSON.stringify({ registry: config.registry, hasToken: Boolean(config.token) }, null, 2)}\n`);
}
