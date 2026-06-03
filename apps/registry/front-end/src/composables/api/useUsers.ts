// src/composables/api/useUsers.ts
import { useQuery } from '@fuyeor/vue-query';
import { type Ref } from 'vue';
import {
  getUserProfile,
  getUserOrganizations,
  getUserPackages,
} from '@/api/users';
import type { User, UserOrganization, UserPackage } from '@/types/user';

/**
 * Fetch public user profile with strict caching and reactive refetching.
 */
export function useUserProfile(usernameRef: Ref<string>) {
  const query = useQuery<User, Error>({
    queryKey: ['user', 'profile', usernameRef],
    queryFn: () => getUserProfile(usernameRef.value),
  });

  return query;
}

/**
 * Fetch public organizations of a user.
 * Lazy-loaded: Query only runs when the component is mounted or active.
 */
export function useUserOrganizations(usernameRef: Ref<string>) {
  return useQuery<UserOrganization[], Error>({
    queryKey: ['user', 'organizations', usernameRef],
    queryFn: () => getUserOrganizations(usernameRef.value),
  });
}

/**
 * Fetch public packages of a user.
 * Lazy-loaded: Query only runs when active.
 */
export function useUserPackages(usernameRef: Ref<string>) {
  return useQuery<UserPackage[], Error>({
    queryKey: ['user', 'packages', usernameRef],
    queryFn: () => getUserPackages(usernameRef.value),
  });
}
