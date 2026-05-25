// src/api/users.ts
import apiClient from './index';
import type { EmbeddedUser } from '@/types/user';

/**
 * Get current user profile
 */
export const getMe = () => {
  return apiClient.get<EmbeddedUser>('/users/me');
};
