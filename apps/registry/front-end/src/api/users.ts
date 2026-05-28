// src/api/users.ts
import apiClient from './index';
import type { User,EmbeddedUser } from '@/types/user';


/**
 * Get current user profile
 */
export const getMe = () => {
  return apiClient.get<EmbeddedUser>('/users/me');
};

/**
 * Public API: Fetch a user profile by their case-insensitive username
 */
export const getUserProfile = (username: string) => {
  return apiClient.get<User>(`/users/${username}`);
};