// src/api/organizations.ts
import apiClient from './index';
import type {
  CheckScopeRequest,
  ScopeValidationResponse,
  CreateScopeRequest,
  CreateScopeResponse,
  OrganizationProfile,
  OrganizationMember,
  OrganizationPackage,
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

/** Fetch public organization metadata by case-insensitive username. */
export const getOrganizationProfile = (username: string) => {
  return apiClient.get<OrganizationProfile>(`/organizations/${encodeURIComponent(username)}`);
};

/** Fetch public organization members and their roles. */
export const getOrganizationMembers = (username: string) => {
  return apiClient.get<OrganizationMember[]>(
    `/organizations/${encodeURIComponent(username)}/members`,
  );
};

/** Fetch public package summaries belonging to an organization. */
export const getOrganizationPackages = (username: string) => {
  return apiClient.get<OrganizationPackage[]>(
    `/organizations/${encodeURIComponent(username)}/packages`,
  );
};
