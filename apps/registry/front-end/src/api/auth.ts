// src/api/auth.ts
import apiClient from './index';
import type {
  SigninRequest,
  SigninResponse,
  CreateTokenRequest,
} from '@/types/auth';

export const signin = (body: SigninRequest) => {
  return apiClient.post<SigninResponse>('/auth/signin', body);
};

/**
 * Manually refresh tokens via HttpOnly Cookie
 */
export const refresh = () => {
  return apiClient.post<SigninResponse>('/auth/refresh');
};

/**
 * Create a CLI token
 */
export const createToken = (body: CreateTokenRequest) => {
  return apiClient.post('/auth/token', body);
};
