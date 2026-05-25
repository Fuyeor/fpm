// @/composables/auth/useAuth.ts
import { computed, unref, type MaybeRef } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { useUserStore } from '@/stores/user';
import type { EmbeddedUser } from '@/types/user';

/**
 * 认证与当前用户状态组合式函数。
 * 它封装了所有与认证和当前用户相关的逻辑，是全站的唯一真实来源。
 */
export function useAuth() {
  const authStore = useAuthStore();
  const userStore = useUserStore();

  // 是否已认证
  const isAuthenticated = computed(() => authStore.isAuthenticated);

  // 当前登录的用户信息 (来自 userStore)
  const currentUser = computed<EmbeddedUser | null>(() => userStore.currentUser);

  /**
   * @param usernameToCheck - 需要检查的用户名，可以是 ref、computed ref 或普通字符串。
   * @returns 一个响应式的 boolean ref，表示是否为当前用户。
   */
  const isCurrentUser = (usernameToCheck: MaybeRef<string>) => {
    return computed(() => {
      // 必须登录
      if (!isAuthenticated.value || !currentUser.value?.username) {
        return false;
      }

      const usernameToCompare = unref(usernameToCheck);

      // 传入的用户名不能为空
      if (!usernameToCompare) return false;

      // 不区分大小写的严格比较
      return currentUser.value.username.toLowerCase() === usernameToCompare.toLowerCase();
    });
  };

  return {
    isAuthenticated,
    currentUser,
    isCurrentUser,
  };
}
