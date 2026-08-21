<!-- @/components/Organization/Content.vue -->
<template>
  <div class="organization-content">
    <Tabs
      container-class="organization"
      :tabs="organizationPageOptions"
      :active-tab-value="currentTab"
    />
    <KeepAlive :max="3">
      <component
        :is="getComponentForTab(currentTab)"
        :key="currentTab"
        :organization="organization"
        :username="username"
      />
    </KeepAlive>
  </div>
</template>

<script setup lang="ts">
import { computed, defineAsyncComponent } from 'vue';
import { useLocale } from '@fuyeor/locale';
import { useLocalePath } from '@/composables/useLocalePath';
import { Tabs, type TabItem } from '@fuyeor/interactify';
import type { OrganizationProfile } from '@/types/organization';

const OrganizationPackages = defineAsyncComponent(
  () => import('@/components/Organization/Tab/Packages.vue'),
);
const OrganizationMembers = defineAsyncComponent(
  () => import('@/components/Organization/Tab/Members.vue'),
);
const OrganizationAbout = defineAsyncComponent(
  () => import('@/components/Organization/Tab/About.vue'),
);

const props = withDefaults(
  defineProps<{
    organization: OrganizationProfile;
    username: string;
    tab?: string;
  }>(),
  { tab: 'packages' },
);
const { t } = useLocale();
const localePath = useLocalePath();
const currentTab = computed(() => props.tab || 'packages');
const basePath = computed(() => localePath(`/organization/@${props.username}`));

const organizationPageOptions = computed<TabItem[]>(() => [
  { labelKey: 'packages', value: 'packages', path: basePath.value },
  {
    labelKey: 'members',
    value: 'members',
    path: `${basePath.value}/members`,
  },
  { labelKey: 'about', value: 'about', path: `${basePath.value}/about` },
]);

const getComponentForTab = (tab: string) => {
  if (tab === 'members') return OrganizationMembers;
  if (tab === 'about') return OrganizationAbout;
  return OrganizationPackages;
};
</script>

<style scoped>
.organization-content {
  display: grid;
  gap: 1.75rem;
  padding: 2rem 0 4rem;
}
</style>
