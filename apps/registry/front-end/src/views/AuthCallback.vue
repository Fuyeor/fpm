<!-- @/views/AuthCallback.vue -->
<template>
  <StateDisplay :loading-text="t('signin.callback.waiting')" type="loading" />
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useRoute, useRouter } from '@fuyeor/vue-router';
import { useLocale } from '@fuyeor/locale';
import { StateDisplay } from '@fuyeor/interactify';
import { signin } from '@/api/auth';
import { useAuthStore } from '@/stores/auth';

const { t } = useLocale();

const route = useRoute();
const router = useRouter();
const authStore = useAuthStore();

onMounted(async () => {
  const code = route.query.code as string;
  const state = route.query.state as string;

  const savedState = window.localStorage.getItem('oauth_state');
  window.localStorage.removeItem('oauth_state');

  if (!savedState || savedState !== state) {
    router.push({ name: 'Welcome' });
    return;
  }

  // 如果拿到 code，就开始换取 Token
  if (code) {
    try {
      const user = await signin({ code });

      authStore.onSigninSuccess(user);

      // 登录成功！跳转到首页！
      router.push({ name: 'Home' });
    } catch {
      router.push({ name: 'Welcome' });
    }
  }
});
</script>
