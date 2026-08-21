// src/api/search.ts
import apiClient from './index';
import type {
  PackageSearchQuery,
  PackageSearchResponse,
} from '@/types/search';

/** Fetches public package search results from the registry search endpoint. */
export const searchPackages = (params: PackageSearchQuery = {}) => {
  const searchParams = new window.URLSearchParams();
  if (params.q !== undefined) searchParams.set('q', params.q);
  if (params.text !== undefined) searchParams.set('text', params.text);
  if (params.limit !== undefined) searchParams.set('limit', String(params.limit));
  if (params.size !== undefined) searchParams.set('size', String(params.size));
  if (params.offset !== undefined) searchParams.set('offset', String(params.offset));
  if (params.from !== undefined) searchParams.set('from', String(params.from));

  const query = searchParams.toString();
  return apiClient.get<PackageSearchResponse>(`/search${query ? `?${query}` : ''}`);
};
