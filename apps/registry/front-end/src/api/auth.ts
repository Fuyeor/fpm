// src/api/auth.ts
import apiClient from './index';
import type { paths } from '@/types/api';
import type { ExtractRequestBody, ExtractResponse } from '@/types/api-helpers';


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
export type CreateTokenRequest = ExtractRequestBody<paths['/auth/token']['post']>;
export const createToken = (body: CreateTokenRequest) => {
  return apiClient.post('/auth/token', body);
};