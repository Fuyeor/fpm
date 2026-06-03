// src/stores/user.ts
import { ref, computed } from 'vue';
import { defineStore, getCookie } from '@fuyeor/commons';
import { getMe } from '@/api/users';
import type { EmbeddedUser, UserOrganization } from '@/types/user';

export const useUserStore = defineStore('user', () => {
  // 防并发请求的 Promise 锁
  let sessionPromise: Promise<void> | null = null;

  // 1. State
  const isLoading = ref<boolean>(false);
  const currentUser = ref<EmbeddedUser | null>(null);
  const organizations = ref<UserOrganization[]>([]);

  // 2. Getters (所有的状态判定，完全基于 currentUser 这一处真理)
const isAuthenticated = computed(() => !!currentUser.value || !!getCookie('session_payload'));

  const isUserLoaded = computed(() => !!currentUser.value);
  
  const canCreateOrganization = computed(() => {
    const ownedCount = organizations.value.filter(org => org.role === 'admin').length;
    return ownedCount < 5;
  });

  // 3. Actions
  
  /**
   * 登录成功的统一处理
   */
  function onSigninSuccess(user: EmbeddedUser, orgs: UserOrganization[]) {
    currentUser.value = user;
    organizations.value = orgs;
  }

  /**
   * 彻底清理用户会话，直接强制回首页
   */
  function signout() {
    currentUser.value = null;
    organizations.value = [];
    window.location.href = '/';
  }

  /**
   * 撞门：自动加载当前登录会话 (极速、防并发)
   */
  async function loadCurrentSession() {
    if (sessionPromise) return sessionPromise;

    isLoading.value = true;
    try {
      sessionPromise = (async () => {
        // ✨ 修正：直接获取返回值，没有任何该死的 .data 套娃！
        const session = await getMe(); 
        onSigninSuccess(session.user, session.organizations);
      })();
      await sessionPromise;
    } catch (error) {
      console.error('[UserStore] Failed to load user session:', error);
      // 如果 401，说明没有登录态，直接清理
      clearSession();
      throw error;
    } finally {
      isLoading.value = false;
      sessionPromise = null;
    }
  }

  function clearSession() {
    currentUser.value = null;
    organizations.value = [];
  }

  return {
    // State & Getters
    isLoading,
    currentUser,
    organizations,
    isAuthenticated,
    isUserLoaded,
    canCreateOrganization,
    
    // Actions
    onSigninSuccess,
    loadCurrentSession,
    signout,
  };
});