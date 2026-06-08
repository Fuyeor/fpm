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
  /** User's organizations */
  organizations: UserOrganization[];
}

export interface User {
  /** UUID v7 string */
  id: string;
  /** Case-insensitive unique username */
  username: string;
  /** Display name */
  nickname: string;
  /** Avatar URL if set */
  avatar?: string;
}

export interface UserOrganization {
  /** Organization UUID */
  id: string;
  /** Scope name, e.g., 'fuyeor' */
  username: string;
  /** Role in this scope: 'admin' | 'publisher' */
  role: string;
  /** ISO timestamp of when the user joined this organization */
  createdAt: string;
}

export interface UserPackage {
  id: string;
  name: string;
  fullName: string;
  description?: string;
  createdAt: string;
}

/**
 * Full session payload returned by /api/users/me (Private cockpit)
 */
export interface MySessionResponse {
  user: EmbeddedUser;
  organizations: UserOrganization[];
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
