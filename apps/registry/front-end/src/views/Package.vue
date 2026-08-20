<!-- @/views/Package.vue -->
<template>
  <HeaderBar :title="t('packages')" />
  <StateDisplay
    v-if="!isRetrieved"
    :type="status"
    :not-found-title="t('user.notFound')"
    :not-found-message="t('user.notFound.desc')"
    :error-message="error?.message"
    @action="router.push({ name: 'Home' })"
  />
  <main v-else class="package-layout">
    <PackageHeader :metadata="metadata" />
    <PackageContent
      :metadata="metadata"
      :scope="scope"
      :name="name"
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
import PackageHeader from '@/components/Package/Header.vue';
import PackageContent from '@/components/Package/Content.vue';
import { usePackageMetadata } from '@/composables/api/usePackages';

const props = defineProps<{
  scope: string;
  name: string;
  tab?: string;
}>();
const router = useRouter();
const titleStore = useTitleStore();
const { t } = useLocale();
const scopeRef = toRef(props, 'scope');
const nameRef = toRef(props, 'name');
const { data: metadata, status, error, isRetrieved } = usePackageMetadata(
  scopeRef,
  nameRef,
);

watch(
  metadata,
  (nextMetadata) => {
    if (nextMetadata) titleStore.setDynamicSegment(nextMetadata.name);
  },
  { immediate: true },
);

onUnmounted(() => {
  titleStore.clearDynamicSegment();
});
</script>

<style scoped>
.package-layout {
  display: grid;
  max-width: 1120px;
  margin: 0 auto;
  padding: 2rem 2rem 0;
}
@media (width <= 640px) {
  .package-layout {
    padding: 1rem 1rem 0;
  }
}
</style>
