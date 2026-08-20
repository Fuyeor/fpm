// src/types/package.d.ts

/** Distribution artifacts for one published package version. */
export interface PackageDist {
  tarball: string;
  shasum: string;
  integrity?: string;
}

/** Abbreviated npm metadata for one package version. */
export interface PackageVersionMetadata {
  name: string;
  version: string;
  dependencies?: Record<string, string>;
  optionalDependencies?: Record<string, string>;
  peerDependencies?: Record<string, string>;
  peerDependenciesMeta?: Record<string, { optional?: boolean }>;
  bin?: Record<string, string> | string;
  engines?: Record<string, string>;
  os?: string[];
  cpu?: string[];
  deprecated?: string;
  bundleDependencies?: string[];
  acceptDependencies?: Record<string, string>;
  directories?: Record<string, string>;
  funding?: unknown;
  hasInstallScript?: boolean;
  dist: PackageDist;
}

/** Top-level abbreviated npm-compatible package metadata. */
export interface PackageMetadata {
  name: string;
  modified: string;
  'dist-tags': {
    latest: string;
  };
  versions: Record<string, PackageVersionMetadata>;
}
