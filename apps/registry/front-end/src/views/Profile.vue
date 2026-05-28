<!-- @/views/Profile.vue -->
<template>
  <HeaderBar :title="t('profile')" />

  <StateDisplay
    v-if="!isRetrieved"
    :type="status"
    :not-found-title="t('user.notFound')"
    :not-found-message="t('user.notFound.desc')"
    @action="router.push({ name: 'Home' })"
  />

  <div v-else-if="user" class="profile-layout">
    <!-- 头部区域 -->
    <ProfileHeader :user="user" />
  </div>
</template>

<script setup lang="ts">
import ProfileHeader from '@/components/Profile/Header.vue';

import { toRef, onUnmounted, watch } from 'vue';
import { useRouter } from '@fuyeor/vue-router';
import { useLocale } from '@fuyeor/locale';
import { useTitleStore } from '@fuyeor/commons';
import { HeaderBar, StateDisplay } from '@fuyeor/interactify';
import { useUserProfile } from '@/composables/api/useUsers';

const props = defineProps<{
  username: string;
  tab?: string;
}>();

const router = useRouter();
const titleStore = useTitleStore();
const usernameRef = toRef(props, 'username');
const { t } = useLocale();

const { data: user, status, isRetrieved } = useUserProfile(usernameRef);

watch(
  user,
  (newUser) => {
    if (newUser) {
      titleStore.setDynamicSegment(
        `${newUser.nickname} (@${newUser.username})`,
      );
    }
  },
  { immediate: true },
);

onUnmounted(() => {
  titleStore.clearDynamicSegment();
});
</script>
