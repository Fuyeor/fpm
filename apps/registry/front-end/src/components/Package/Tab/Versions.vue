<!-- @/components/Package/Tab/Versions.vue -->
<template>
  <section class="package-tab" aria-labelledby="package-versions-title">
    <header class="tab-heading">
      <div>
        <p class="eyebrow">{{ t('package.versions') }}</p>
        <h2 id="package-versions-title">{{ t('package.metadata') }}</h2>
      </div>
      <span class="count-badge">{{ versions.length }}</span>
    </header>
    <div v-if="versions.length" class="version-list">
      <article v-for="version in versions" :key="version.version" class="version-card">
        <div>
          <h3>{{ version.version }}</h3>
          <p>{{ metadata.name }}</p>
        </div>
        <div class="version-details">
          <span v-if="version.version === latestVersion" class="latest-badge">
            {{ t('package.latest') }}
          </span>
          <a
            class="dist-link"
            :href="version.dist.tarball"
            target="_blank"
            rel="noreferrer"
          >
            tarball
          </a>
        </div>
      </article>
    </div>
    <EmptyCard v-else :title="t('package.noVersions')" />
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
const versions = computed(() =>
  Object.values(props.metadata.versions).sort((left, right) =>
    right.version.localeCompare(left.version, undefined, { numeric: true }),
  ),
);
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
  gap: 1rem;
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
  font-size: 1.05rem;
  font-weight: 500;
}
.count-badge,
.latest-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 999px;
  background: var(--surface-raised-hover);
  color: var(--text-secondary);
  font-size: 0.78rem;
  font-weight: 700;
}
.count-badge {
  min-width: 2rem;
  height: 2rem;
  padding: 0 0.5rem;
}
.version-list {
  display: grid;
  gap: 0.75rem;
}
.version-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 1rem 1.25rem;
  border: var(--border-default);
  border-radius: var(--radius-md);
  background: var(--surface-top);
}
.version-card h3,
.version-card p {
  margin: 0;
}
.version-card h3 {
  color: var(--text-primary);
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 0.98rem;
}
.version-card p {
  margin-top: 0.35rem;
  color: var(--text-secondary);
  font-size: 0.8rem;
}
.version-details {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: flex-end;
  gap: 0.5rem;
}
.latest-badge {
  padding: 0.35rem 0.6rem;
  background: rgb(14 165 233 / 12%);
  color: #0284c7;
}
.dist-link {
  color: var(--text-secondary);
  font-size: 0.8rem;
  text-decoration: none;
}
.dist-link:hover {
  color: var(--text-primary);
  text-decoration: underline;
}
@media (width <= 560px) {
  .version-card {
    align-items: flex-start;
    flex-direction: column;
  }
  .version-details {
    justify-content: flex-start;
  }
}
</style>
