// src/composables/api/useUsers.ts
import { useQuery } from '@fuyeor/vue-query';
import { type Ref } from 'vue';
import { getUserProfile } from '@/api/users';
import type { User } from '@/types/user';

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
