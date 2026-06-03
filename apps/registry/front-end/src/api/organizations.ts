// src/api/organizations.ts
import apiClient from './index';
import type {
  CheckScopeRequest,
  ScopeValidationResponse,
  CreateScopeRequest,
  CreateScopeResponse,
} from '@/types/organization';

/**
 * Check if a scope name is available (Anonymous/Authenticated)
 */
export const validateScope = (body: CheckScopeRequest) => {
  return apiClient.post<ScopeValidationResponse>('/organizations/validation', body);
};

/**
 * Create a new Scope (Organization) (Authenticated only)
 */
export const createOrganization = (body: CreateScopeRequest) => {
  return apiClient.post<CreateScopeResponse>('/organizations', body);
};
