// packages/cli/test/api.test.ts
import { afterEach, describe, expect, it, vi } from 'vitest';
import { acquireUpload, commitUpload, uploadTarball } from '../src/api.js';
import type { FpmConfig } from '../src/types.js';

const config: FpmConfig = { registry: 'https://registry.example/v1', token: 'fpm_test-token' };

afterEach(() => vi.restoreAllMocks());

describe('registry API', () => {
  it('runs the two-phase upload flow with Bearer authentication', async () => {
    const fetchMock = vi.spyOn(globalThis, 'fetch')
      .mockResolvedValueOnce(new Response(JSON.stringify({ uploadUrl: 'https://r2.example/upload', uploadSessionId: 'session' }), { status: 200 }))
      .mockResolvedValueOnce(new Response('', { status: 200 }))
      .mockResolvedValueOnce(new Response('', { status: 201 }));

    const manifest = { name: '@demo/source-package', version: '1.2.3' };
    const acquired = await acquireUpload(config, manifest, 'a'.repeat(64));
    await uploadTarball(acquired.uploadUrl, new Uint8Array([1, 2, 3]));
    await commitUpload(config, acquired.uploadSessionId, manifest);

    expect(fetchMock).toHaveBeenCalledTimes(3);
    expect(fetchMock.mock.calls[0]?.[0]).toBe('https://registry.example/v1/packages/acquire');
    const acquireHeaders = new Headers((fetchMock.mock.calls[0]?.[1] as RequestInit).headers);
    expect(acquireHeaders.get('Authorization')).toBe('Bearer fpm_test-token');
    expect((fetchMock.mock.calls[1]?.[1] as RequestInit).method).toBe('PUT');
    expect(fetchMock.mock.calls[1]?.[0]).toBe('https://r2.example/upload');
    expect(fetchMock.mock.calls[2]?.[0]).toBe('https://registry.example/v1/packages/commit');
  });
});
