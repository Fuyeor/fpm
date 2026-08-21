<!-- @/components/Package/Header.vue -->
<template>
  <section class="package-header" aria-labelledby="package-name">
    <div class="package-mark" aria-hidden="true">&lt;/&gt;</div>
    <div class="package-heading">
      <p class="eyebrow">{{ t('package.metadata') }}</p>
      <h1 id="package-name">{{ metadata.name }}</h1>
      <p class="package-meta">
        <span>{{ t('package.latest') }} <strong>{{ latestVersion }}</strong></span>
        <span class="meta-divider">·</span>
        <span>{{ metadata.versionsCount }} versions</span>
      </p>
    </div>
    <div class="install-command">
      <span>{{ t('package.install') }}</span>
      <code>pnpm add {{ metadata.name }}</code>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useLocale } from '@fuyeor/locale';
import type { PackageMetadata } from '@/types/package';

const props = defineProps<{
  metadata: PackageMetadata;
}>();
const { t } = useLocale();
const latestVersion = computed(() => props.metadata['dist-tags'].latest);
const metadata = computed(() => ({
  ...props.metadata,
  versionsCount: Object.keys(props.metadata.versions).length,
}));
</script>

<style scoped>
.package-header {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr) minmax(250px, auto);
  gap: 1.5rem;
  align-items: center;
  padding: 1.5rem;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--surface-top);
}
.package-mark {
  display: grid;
  width: 76px;
  height: 76px;
  place-items: center;
  border-radius: 20px;
  background: #0f172a;
  color: #67e8f9;
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 1.35rem;
  font-weight: 700;
}
.eyebrow {
  margin: 0 0 0.4rem;
  color: var(--text-secondary);
  font-size: 0.76rem;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}
.package-heading {
  min-width: 0;
}
.package-heading h1 {
  margin: 0;
  color: var(--text-primary);
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: clamp(1.25rem, 3vw, 1.8rem);
  overflow-wrap: anywhere;
}
.package-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin: 0.65rem 0 0;
  color: var(--text-secondary);
  font-size: 0.86rem;
}
.meta-divider {
  color: var(--border-strong, var(--text-secondary));
}
.install-command {
  display: grid;
  gap: 0.45rem;
  justify-items: end;
}
.install-command span {
  color: var(--text-secondary);
  font-size: 0.78rem;
  font-weight: 700;
  text-transform: uppercase;
}
.install-command code {
  padding: 0.7rem 0.85rem;
  border: var(--border-default);
  border-radius: var(--radius-sm);
  background: var(--surface-raised-hover);
  color: var(--text-primary);
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 0.82rem;
}
@media (width <= 960px) {
  .package-header {
    grid-template-columns: auto minmax(0, 1fr);
  }
  .install-command {
    grid-column: 1 / -1;
    justify-items: stretch;
  }
}
@media (width <= 480px) {
  .package-header {
    gap: 1rem;
    padding: 1rem;
  }
  .package-mark {
    width: 56px;
    height: 56px;
    border-radius: 15px;
    font-size: 1rem;
  }
}
</style>
