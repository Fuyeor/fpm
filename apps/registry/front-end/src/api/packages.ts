// src/api/packages.ts
import apiClient from './index';
import type { PackageMetadata } from '@/types/package';

/** Fetches npm-compatible abbreviated metadata for a scoped package. */
export const getPackageMetadata = (scope: string, name: string) => {
  return apiClient.get<PackageMetadata>(
    `/${encodeURIComponent(scope)}/${encodeURIComponent(name)}`,
  );
};
