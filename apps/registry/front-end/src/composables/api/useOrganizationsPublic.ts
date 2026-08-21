// src/composables/api/useOrganizationsPublic.ts
import { useQuery } from '@fuyeor/vue-query';
import type { Ref } from 'vue';
import {
  getOrganizationMembers,
  getOrganizationPackages,
  getOrganizationProfile,
} from '@/api/organizations';
import type {
  OrganizationMember,
  OrganizationPackage,
  OrganizationProfile,
} from '@/types/organization';

/** Fetches public organization metadata and keeps it reactive to the route username. */
export function useOrganizationProfile(usernameRef: Ref<string>) {
  return useQuery<OrganizationProfile, Error>({
    queryKey: ['organization', 'profile', usernameRef],
    queryFn: () => getOrganizationProfile(usernameRef.value),
  });
}

/** Fetches the public organization member list. */
export function useOrganizationMembers(usernameRef: Ref<string>) {
  return useQuery<OrganizationMember[], Error>({
    queryKey: ['organization', 'members', usernameRef],
    queryFn: () => getOrganizationMembers(usernameRef.value),
  });
}

/** Fetches the public organization package summary list. */
export function useOrganizationPackages(usernameRef: Ref<string>) {
  return useQuery<OrganizationPackage[], Error>({
    queryKey: ['organization', 'packages', usernameRef],
    queryFn: () => getOrganizationPackages(usernameRef.value),
  });
}
