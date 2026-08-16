// packages/cli/src/types.ts
export interface PackageManifest {
  name: string;
  version: string;
  description?: string;
  files?: string[];
  [key: string]: unknown;
}

export interface UploadAcquireResponse {
  uploadUrl: string;
  uploadSessionId: string;
}

export interface PublishResult {
  name: string;
  version: string;
  tarball: string;
  sha256: string;
}

export interface FpmConfig {
  registry: string;
  token?: string;
}

export interface UserProfile {
  id: string;
  username: string;
  nickname: string;
  avatar?: string | null;
}
