// src/composables/api/usePackages.ts
import { useQuery } from '@fuyeor/vue-query';
import type { Ref } from 'vue';
import { getPackageMetadata } from '@/api/packages';
import type { PackageMetadata } from '@/types/package';

/** Fetches npm-compatible metadata for a route-scoped package. */
export function usePackageMetadata(scopeRef: Ref<string>, nameRef: Ref<string>) {
  return useQuery<PackageMetadata, Error>({
    queryKey: ['package', 'metadata', scopeRef, nameRef],
    queryFn: () => getPackageMetadata(scopeRef.value, nameRef.value),
  });
}
