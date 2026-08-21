<!-- @/components/Package/Content.vue -->
<template>
  <div class="package-content">
    <Tabs
      container-class="package"
      :tabs="packagePageOptions"
      :active-tab-value="currentTab"
    />
    <KeepAlive :max="3">
      <component
        :is="getComponentForTab(currentTab)"
        :key="currentTab"
        :metadata="metadata"
      />
    </KeepAlive>
  </div>
</template>

<script setup lang="ts">
import { computed, defineAsyncComponent } from 'vue';
import { useLocale } from '@fuyeor/locale';
import { Tabs, type TabItem } from '@fuyeor/interactify';
import { useLocalePath } from '@/composables/useLocalePath';
import type { PackageMetadata } from '@/types/package';

const PackageVersions = defineAsyncComponent(
  () => import('@/components/Package/Tab/Versions.vue'),
);
const PackageDependencies = defineAsyncComponent(
  () => import('@/components/Package/Tab/Dependencies.vue'),
);
const PackageDependents = defineAsyncComponent(
  () => import('@/components/Package/Tab/Dependents.vue'),
);

const props = withDefaults(
  defineProps<{
    metadata: PackageMetadata;
    scope: string;
    name: string;
    tab?: string;
  }>(),
  { tab: 'versions' },
);
const { t } = useLocale();
const localePath = useLocalePath();
const currentTab = computed(() => props.tab || 'versions');
const basePath = computed(
  () => localePath(`/package/${props.scope}/${props.name}`),
);
const packagePageOptions = computed<TabItem[]>(() => [
  {
    labelKey: 'package.versions',
    value: 'versions',
    path: basePath.value,
  },
  {
    labelKey: 'package.dependencies',
    value: 'dependencies',
    path: `${basePath.value}/dependencies`,
  },
  {
    labelKey: 'package.dependents',
    value: 'dependents',
    path: `${basePath.value}/dependents`,
  },
]);

const getComponentForTab = (tab: string) => {
  if (tab === 'dependencies') return PackageDependencies;
  if (tab === 'dependents') return PackageDependents;
  return PackageVersions;
};
</script>

<style scoped>
.package-content {
  display: grid;
  gap: 1.75rem;
  padding: 2rem 0 4rem;
}
</style>
