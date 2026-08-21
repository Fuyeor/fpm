<!-- @/components/Package/Tab/Dependencies.vue -->
<template>
  <section class="package-tab" aria-labelledby="package-dependencies-title">
    <header class="tab-heading">
      <div>
        <p class="eyebrow">{{ t('package.dependencies') }}</p>
        <h2 id="package-dependencies-title">{{ latestVersion }}</h2>
      </div>
    </header>
    <div v-if="dependencyGroups.length" class="dependency-groups">
      <section
        v-for="group in dependencyGroups"
        :key="group.label"
        class="dependency-group"
      >
        <h3>{{ group.label }}</h3>
        <div class="dependency-list">
          <div
            v-for="([name, range]) in Object.entries(group.values)"
            :key="name"
            class="dependency-item"
          >
            <code>{{ name }}</code>
            <span>{{ range }}</span>
          </div>
        </div>
      </section>
    </div>
    <EmptyCard v-else :title="t('package.noDependencies')" />
  </section>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useLocale } from '@fuyeor/locale';
import EmptyCard from '@/components/Option/Card/Empty.vue';
import type { PackageMetadata } from '@/types/package';

const props = defineProps<{
  metadata: PackageMetadata;
}>();
const { t } = useLocale();
const latestVersion = computed(() => props.metadata['dist-tags'].latest);
const latestMetadata = computed(
  () => props.metadata.versions[latestVersion.value],
);
const dependencyGroups = computed(() => {
  const latest = latestMetadata.value;
  if (!latest) return [];
  return [
    { label: t('package.dependencies'), values: latest.dependencies },
    { label: 'Optional dependencies', values: latest.optionalDependencies },
    { label: 'Peer dependencies', values: latest.peerDependencies },
  ].filter((group) => group.values && Object.keys(group.values).length > 0) as Array<{
    label: string;
    values: Record<string, string>;
  }>;
});
</script>

<style scoped>
.package-tab {
  display: grid;
  gap: 1.25rem;
}
.tab-heading {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
}
.eyebrow {
  margin: 0 0 0.45rem;
  color: var(--text-secondary);
  font-size: 0.76rem;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}
.tab-heading h2 {
  margin: 0;
  color: var(--text-primary);
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 1.05rem;
}
.dependency-groups {
  display: grid;
  gap: 1rem;
}
.dependency-group {
  padding: 1.25rem;
  border: var(--border-default);
  border-radius: var(--radius-md);
  background: var(--surface-top);
}
.dependency-group h3 {
  margin: 0 0 0.85rem;
  color: var(--text-primary);
  font-size: 0.9rem;
}
.dependency-list {
  display: grid;
  gap: 0.35rem;
}
.dependency-item {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  padding: 0.6rem 0;
  border-bottom: var(--border-default);
  color: var(--text-secondary);
  font-size: 0.84rem;
}
.dependency-item:last-child {
  border-bottom: 0;
}
.dependency-item code {
  color: var(--text-primary);
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
}
@media (width <= 560px) {
  .dependency-item {
    align-items: flex-start;
    flex-direction: column;
    gap: 0.2rem;
  }
}
</style>
