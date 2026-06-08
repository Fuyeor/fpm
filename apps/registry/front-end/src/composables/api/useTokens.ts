// src/composables/api/useTokens.ts
import { useQuery, useMutation, useQueryClient } from '@fuyeor/vue-query';
import { createToken, getTokens, deleteToken } from '@/api/auth';
import type {
  CreateTokenRequest,
  CreateTokenResponse,
  UserToken,
} from '@/types/auth';

/**
 * get token list
 */
export function useTokensQuery() {
  return useQuery<UserToken[], Error>({
    queryKey: ['user', 'tokens'],
    queryFn: getTokens,
  });
}

/**
 * generate new token
 */
export function useCreateTokenMutation() {
  const queryClient = useQueryClient();

  return useMutation<CreateTokenResponse, Error, CreateTokenRequest>({
    mutationFn: createToken,
    onSuccess: () => {
      // 成功后仅使 tokens 列表失效并触发刷新
      queryClient.invalidateQueries({ queryKey: ['user', 'tokens'] });
    },
  });
}

/**
 * revoke token
 */
export function useRevokeTokenMutation() {
  const queryClient = useQueryClient();

  return useMutation<void, Error, string>({
    mutationFn: deleteToken,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['user', 'tokens'] });
    },
  });
}
