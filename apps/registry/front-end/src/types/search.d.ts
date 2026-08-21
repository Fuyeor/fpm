// src/types/search.d.ts

/** Query parameters accepted by the public package search endpoint. */
export interface PackageSearchQuery {
  q?: string;
  text?: string;
  limit?: number;
  size?: number;
  offset?: number;
  from?: number;
}

/** Stable package links returned by search. */
export interface PackageSearchLinks {
  npm: string;
}

/** Public package summary returned by search. */
export interface PackageSearchPackage {
  name: string;
  version: string;
  description: string | null;
  date: string;
  links: PackageSearchLinks;
}

/** npm-compatible search result item. */
export interface PackageSearchObject {
  package: PackageSearchPackage;
  score: {
    finalScore: number;
    detail: {
      quality: number;
      popularity: number;
      maintenance: number;
    };
  };
  searchScore: number;
}

/** npm-compatible package search response. */
export interface PackageSearchResponse {
  objects: PackageSearchObject[];
  total: number;
  time: string;
}
