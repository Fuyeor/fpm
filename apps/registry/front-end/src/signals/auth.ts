// src/signals/auth.ts
import { Signal } from 'signal-polyfill';
import { EmbeddedUser } from '@/types/user';

/**
 * The current user info. Managed by signin and refresh.
 */
export const currentUser = new Signal.State<EmbeddedUser | null>(null);

/**
 * Computed signal to check if authenticated
 */
export const isAuthenticated = {
  get: () => currentUser.get() !== null,
};
