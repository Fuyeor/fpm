// src/services/auth.service.ts
import { signin, refresh } from '@/api/auth';
import { getMe } from '@/api/user';
import { currentUser } from '@/signals/auth';

/**
 * Redirect to IdP for OAuth flow
 */
export const startSigninFlow = () => {
  const url = new URL(import.meta.env.VITE_APP_AUTH_BASE_URL);
  url.searchParams.set('client_id', import.meta.env.VITE_APP_OAUTH_CLIENT_ID);
  url.searchParams.set('redirect_uri', import.meta.env.VITE_APP_REDIRECT_URI);
  url.searchParams.set('response_type', 'code');
  url.searchParams.set('scope', 'openid profile');

  window.location.href = url.toString();
};

/**
 * Standard initialization logic: Try to knocking the door
 */
export const initializeAuth = async () => {
  try {
    const { data } = await getMe();
    currentUser.set(data);
  } catch (err: any) {
    if (err.response?.status === 401) {
      try {
        await refresh();
        const { data: userData } = await getMe();
        currentUser.set(userData);
      } catch {
        currentUser.set(null);
      }
    }
  }
};

/**
 * Handle authorization callback
 */
export const handleCallback = async (code: string) => {
  try {
    const { data } = await signin({ code });
    currentUser.set(data.user);
    window.history.replaceState({}, '', '/');
  } catch (err) {
    console.error('Signin failed:', err);
    window.location.href = '/';
  }
};
