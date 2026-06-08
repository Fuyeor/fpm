// src/api/auth.ts
import apiClient from './index';
import type {
  CreateTokenRequest,
  CreateTokenResponse,
  SigninRequest,
  SigninResponse,
  UserToken,
} from '@/types/auth';

/**
 * Signin via IdP OAuth code
 */
export const signin = (body: SigninRequest) => {
  return apiClient.post<SigninResponse>('/auth/signin', body);
};

/**
 * Manually refresh tokens via HttpOnly Cookie
 */
export const refresh = () => {
  return apiClient.post<SigninResponse>('/auth/refresh');
};

export const getTokens = () => {
  return apiClient.get<UserToken[]>('/auth/tokens');
};

/**
 * Create a CLI token
 */
export const createToken = (body: CreateTokenRequest) => {
  return apiClient.post<CreateTokenResponse>('/auth/token', body);
};

/**
 * Delete a CLI token
 */
export const deleteToken = (id: string) => {
  return apiClient.delete<void>(`/auth/tokens/${id}`);
};
