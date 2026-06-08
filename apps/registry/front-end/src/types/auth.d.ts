// @/types/auth.d.ts
import type { EmbeddedUser } from './user';

/**
 * Request payload for signing in (code exchange)
 */
export interface SigninRequest {
  code: string;
}

/**
 * Response for a successful signin
 */
export interface SigninResponse {
  accessToken: string;
  refreshToken: string;
  user: EmbeddedUser;
}

export interface UserToken {
  id: string;
  name: string;
  createdAt: string;
}

/**
 * Request payload to create a CLI token
 */
export interface CreateTokenRequest {
  name: string;
}

/**
 * Response for a successfully created CLI token
 */
export interface CreateTokenResponse {
  token: string;
}
