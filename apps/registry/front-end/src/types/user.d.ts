// src/types/user.d.ts

export interface EmbeddedUser {
  /** UUID v7 string */
  id: string;
  /** Case-insensitive unique username */
  username: string;
  /** Display name */
  nickname: string;
  /** Avatar URL if set */
  avatar?: string;
}

/**
 * Standard Signin Response (Web & CLI)
 * Contains dual tokens and user profile
 */
export interface SigninResponse {
  /** JWT access token (short-lived) */
  accessToken: string;
  /** JWT refresh token (long-lived) */
  refreshToken: string;
  /** User information */
  user: EmbeddedUser;
}

/**
 * Request payload for signin
 */
export interface SigninRequest {
  /** OAuth code from IdP */
  code: string;
}

/**
 * Response for CLI token creation
 */
export interface CreateTokenResponse {
  /** The plaintext fpm_xxx token */
  token: string;
}
