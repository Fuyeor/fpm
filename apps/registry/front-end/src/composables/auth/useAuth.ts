// @/composables/auth/useAuth.ts
import { computed, unref, type MaybeRef } from 'vue';
import { useUserStore } from '@/stores/user';

/**
 * 认证与当前用户状态组合式函数。
 * 它封装了所有与认证和当前用户相关的逻辑，是全站的唯一真实来源。
 */
export function useAuth() {
  const userStore = useUserStore();

  // 是否已认证
  const isAuthenticated = computed(() => userStore.isAuthenticated);

  // 当前登录的用户信息 (来自 userStore)
  const currentUser = computed(() => userStore.currentUser);

  /**
   * @param id - 需要检查的用户 ID，可以是 ref、computed ref 或普通字符串。
   * @returns 一个响应式的 boolean ref，表示是否为当前用户。
   */
  const checkIsCurrentUser = (id: MaybeRef<string>) => {
    return computed(() => {
      // 必须登录
      if (!currentUser.value) return false;

      const userId = unref(id);

      // 传入的用户 ID 不能为空
      if (!userId) return false;

      // ID 一般不会大写
      return currentUser.value.id === userId;
    });
  };

  return {
    isAuthenticated,
    currentUser,
    checkIsCurrentUser,
  };
}
