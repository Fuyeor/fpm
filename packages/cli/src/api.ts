// packages/cli/src/api.ts
import type { FpmConfig, PackageManifest, PublishResult, UploadAcquireResponse, UserProfile } from './types.js';

export class RegistryError extends Error {
  readonly status: number;
  readonly retryable: boolean;

  constructor(message: string, status: number, retryable = false) {
    super(message);
    this.name = 'RegistryError';
    this.status = status;
    this.retryable = retryable;
  }
}

function endpoint(registry: string, path: string): string {
  return `${registry.replace(/\/+$/, '')}/${path.replace(/^\/+/, '')}`;
}

async function parseError(response: Response): Promise<string> {
  const body = await response.text();
  if (!body) return response.statusText || `HTTP ${response.status}`;
  try {
    const parsed = JSON.parse(body) as { message?: string; error?: string };
    return parsed.message ?? parsed.error ?? body;
  } catch {
    return body;
  }
}

async function request<T>(config: FpmConfig, path: string, init: RequestInit = {}): Promise<T> {
  const headers = new Headers(init.headers);
  headers.set('Accept', 'application/json');
  if (init.body && !headers.has('Content-Type')) headers.set('Content-Type', 'application/json');
  if (config.token) headers.set('Authorization', `Bearer ${config.token}`);

  let response: Response;
  try {
    response = await fetch(endpoint(config.registry, path), { ...init, headers, signal: AbortSignal.timeout(30_000) });
  } catch (error) {
    throw new RegistryError(`Registry request failed: ${(error as Error).message}`, 0, true);
  }
  if (!response.ok) throw new RegistryError(await parseError(response), response.status, response.status >= 500 || response.status === 429);
  if (response.status === 204) return undefined as T;
  const body = await response.text();
  if (!body.trim()) return undefined as T;
  try {
    return JSON.parse(body) as T;
  } catch {
    throw new RegistryError('Registry returned an invalid JSON response.', response.status);
  }
}

export async function health(config: FpmConfig): Promise<void> {
  await request(config, '/health');
}

export async function whoami(config: FpmConfig): Promise<UserProfile> {
  return await request<UserProfile>(config, '/users/me');
}

export async function acquireUpload(config: FpmConfig, manifest: PackageManifest, sha256: string): Promise<UploadAcquireResponse> {
  return await request<UploadAcquireResponse>(config, '/packages/acquire', {
    method: 'POST',
    body: JSON.stringify({ name: manifest.name, version: manifest.version, shasum: sha256 }),
  });
}

export async function uploadTarball(uploadUrl: string, tarball: Uint8Array): Promise<void> {
  for (let attempt = 0; attempt < 2; attempt += 1) {
    try {
      const response = await fetch(uploadUrl, {
        method: 'PUT',
        body: tarball.buffer.slice(tarball.byteOffset, tarball.byteOffset + tarball.byteLength) as ArrayBuffer,
        headers: { 'Content-Type': 'application/gzip' },
        signal: AbortSignal.timeout(60_000),
      });
      if (response.ok) return;
      if (response.status < 500 && response.status !== 429) throw new RegistryError(await parseError(response), response.status);
      if (attempt === 1) throw new RegistryError(await parseError(response), response.status, true);
    } catch (error) {
      if (error instanceof RegistryError && !error.retryable) throw error;
      if (attempt === 1) throw error;
    }
  }
}

export async function commitUpload(config: FpmConfig, uploadSessionId: string, manifest: PackageManifest): Promise<void> {
  await request(config, '/packages/commit', {
    method: 'POST',
    body: JSON.stringify({ uploadSessionId, manifest }),
  });
}

export async function publish(config: FpmConfig, manifest: PackageManifest, tarball: Uint8Array, sha256: string): Promise<PublishResult> {
  const acquired = await acquireUpload(config, manifest, sha256);
  await uploadTarball(acquired.uploadUrl, tarball);
  await commitUpload(config, acquired.uploadSessionId, manifest);
  return { name: manifest.name, version: manifest.version, tarball: `${config.registry}/packages/${manifest.name}/${manifest.version}.tgz`, sha256 };
}
