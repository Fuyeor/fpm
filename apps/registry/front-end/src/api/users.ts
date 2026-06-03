// src/api/user.ts
import apiClient from './index';
import type {
  EmbeddedUser,
  MySessionResponse,
  UserOrganization,
  UserPackage,
} from '@/types/user';

/**
 * Get current user session big-package (Private)
 * Returns { user, organizations }
 */
export const getMe = () => {
  return apiClient.get<MySessionResponse>('/users/me');
};

/**
 * Public API: Fetch a user profile by their case-insensitive username
 * Returns only the pure User metadata (EmbeddedUser)
 */
export const getUserProfile = (username: string) => {
  return apiClient.get<EmbeddedUser>(`/users/${username}`);
};

export const getUserOrganizations = (username: string) => {
  return apiClient.get<UserOrganization[]>(`/users/${username}/organizations`);
};

export const getUserPackages = (username: string) => {
  return apiClient.get<UserPackage[]>(`/users/${username}/packages`);
};
