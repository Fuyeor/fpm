<!-- @/views/Organization.vue -->
<template>
  <HeaderBar :title="t('organizations')" />
  <StateDisplay
    v-if="!isRetrieved"
    :type="status"
    :not-found-title="t('user.notFound')"
    :not-found-message="t('user.notFound.desc')"
    :error-message="error?.message"
    @action="router.push({ name: 'Home' })"
  />
  <main v-else class="organization-layout">
    <OrganizationHeader :organization="organization" />
    <OrganizationContent
      :organization="organization"
      :username="username"
      :tab="tab"
    />
  </main>
</template>

<script setup lang="ts">
import { toRef, onUnmounted, watch } from 'vue';
import { useRouter } from '@fuyeor/vue-router';
import { useLocale } from '@fuyeor/locale';
import { useTitleStore } from '@fuyeor/commons';
import { HeaderBar, StateDisplay } from '@fuyeor/interactify';
import OrganizationHeader from '@/components/Organization/Header.vue';
import OrganizationContent from '@/components/Organization/Content.vue';
import { useOrganizationProfile } from '@/composables/api/useOrganizationsPublic';

const props = defineProps<{
  username: string;
  tab?: string;
}>();
const router = useRouter();
const titleStore = useTitleStore();
const { t } = useLocale();
const usernameRef = toRef(props, 'username');
const {
  data: organization,
  status,
  error,
  isRetrieved,
} = useOrganizationProfile(usernameRef);

watch(
  organization,
  (nextOrganization) => {
    if (nextOrganization) {
      titleStore.setDynamicSegment(`@${nextOrganization.username}`);
    }
  },
  { immediate: true },
);

onUnmounted(() => {
  titleStore.clearDynamicSegment();
});
</script>

<style scoped>
.organization-layout {
  display: grid;
  max-width: 1120px;
  margin: 0 auto;
  padding: 2rem 2rem 0;
}
@media (width <= 640px) {
  .organization-layout {
    padding: 1rem 1rem 0;
  }
}
</style>
