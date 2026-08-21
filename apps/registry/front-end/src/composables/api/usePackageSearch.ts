// src/composables/api/usePackageSearch.ts
import { useQuery } from '@fuyeor/vue-query';
import type { Ref } from 'vue';
import { searchPackages } from '@/api/search';
import type {
  PackageSearchQuery,
  PackageSearchResponse,
} from '@/types/search';

/** Fetches and caches public package search results by normalized query input. */
export function usePackageSearch(queryRef: Ref<PackageSearchQuery>) {
  return useQuery<PackageSearchResponse, Error>({
    queryKey: ['package', 'search', queryRef],
    queryFn: () => searchPackages(queryRef.value),
  });
}
