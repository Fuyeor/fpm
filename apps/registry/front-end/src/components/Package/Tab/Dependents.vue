<!-- @/components/Package/Tab/Dependents.vue -->
<template>
  <section class="package-tab" aria-labelledby="package-dependents-title">
    <p class="eyebrow">{{ t('package.dependents') }}</p>
    <div class="notice-panel">
      <h2 id="package-dependents-title">{{ t('package.dependents') }}</h2>
      <p>{{ t('package.dependentsUnavailable') }}</p>
    </div>
  </section>
</template>

<script setup lang="ts">
import { useLocale } from '@fuyeor/locale';
const { t } = useLocale();
</script>

<style scoped>
.package-tab {
  display: grid;
  gap: 1.25rem;
}
.eyebrow {
  margin: 0;
  color: var(--text-secondary);
  font-size: 0.76rem;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}
.notice-panel {
  padding: 2rem;
  border: var(--border-default);
  border-radius: var(--radius-md);
  background: var(--surface-top);
}
.notice-panel h2 {
  margin: 0;
  color: var(--text-primary);
  font-size: 1.1rem;
}
.notice-panel p {
  margin: 0.6rem 0 0;
  color: var(--text-secondary);
  line-height: 1.6;
}
</style>
