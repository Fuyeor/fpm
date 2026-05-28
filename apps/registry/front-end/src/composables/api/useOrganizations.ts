// src/composables/api/useOrganizations.ts
import { useMutation, useQueryClient } from '@fuyeor/vue-query';
import { validateScope, createOrganization } from '@/api/organizations';
import type { 
  CheckScopeRequest, 
  CreateScopeRequest, 
  ScopeValidationResponse, 
  CreateScopeResponse 
} from '@/types/organization';

/**
 * Composable for managing Scope (Organization) mutations and state.
 * Fully compatible with @fuyeor/vue-query and automatic response unwrapping.
 */
export function useOrganizations() {
  const queryClient = useQueryClient();

  /**
   * Mutation to validate if a scope name is available.
   * Directly passes validateScope thanks to automated API unwrapping!
   */
  const validateScopeMutation = useMutation<ScopeValidationResponse, Error, CheckScopeRequest>({
    mutationFn: validateScope, // ✨ 直接传递函数，极其环保，没有任何套娃
  });

  /**
   * Mutation to create a new Scope (Organization).
   */
  const createOrganizationMutation = useMutation<CreateScopeResponse, Error, CreateScopeRequest>({
    mutationFn: createOrganization, // ✨ 同样直接传递
    onSuccess: (data) => {
      // 成功后自动失效缓存，触发全局重新拉取
      queryClient.invalidateQueries({ queryKey: ['user', 'organizations'] });
      console.log(`[Vue Query] Organization '${data.name}' created.`);
    },
  });

  return {
    // Validate Scope Mutation State
    validateScope: validateScopeMutation.mutateAsync,
    isValidating: validateScopeMutation.isPending,
    validationData: validateScopeMutation.data,
    validationError: validateScopeMutation.error,

    // Create Organization Mutation State
    createOrganization: createOrganizationMutation.mutateAsync,
    isCreating: createOrganizationMutation.isPending,
    creationData: createOrganizationMutation.data,
    creationError: createOrganizationMutation.error,
  };
}