<!-- @/components/Organization/Tab/About.vue -->
<template>
  <section class="organization-about" aria-labelledby="organization-about-title">
    <div class="about-heading">
      <p class="eyebrow">{{ t('about') }}</p>
      <h2 id="organization-about-title">{{ t('organization.about.desc') }}</h2>
    </div>
    <div class="about-panel">
      <p v-if="organization.description" class="about-description">
        {{ organization.description }}
      </p>
      <p v-else class="about-description muted">{{ t('content.empty') }}</p>
      <dl class="about-meta">
        <div>
          <dt>{{ t('organization.created') }}</dt>
          <dd>{{ formatDate(organization.createdAt, { preset: 'full' }) }}</dd>
        </div>
        <div>
          <dt>{{ t('members') }}</dt>
          <dd>{{ t('organization.members.desc') }}</dd>
        </div>
      </dl>
    </div>
  </section>
</template>

<script setup lang="ts">
import { useLocale } from '@fuyeor/locale';
import { useDateFormatter } from '@fuyeor/commons';
import type { OrganizationProfile } from '@/types/organization';

const props = defineProps<{
  organization: OrganizationProfile;
}>();
const { t } = useLocale();
const { formatDate } = useDateFormatter();
</script>

<style scoped>
.organization-about {
  display: grid;
  gap: 1.25rem;
}
.eyebrow {
  margin: 0 0 0.45rem;
  color: var(--text-secondary);
  font-size: 0.76rem;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}
.about-heading h2 {
  max-width: 640px;
  margin: 0;
  color: var(--text-primary);
  font-size: 1.05rem;
  font-weight: 500;
  line-height: 1.5;
}
.about-panel {
  display: grid;
  gap: 2rem;
  padding: 1.5rem;
  border: var(--border-default);
  border-radius: var(--radius-md);
  background: var(--surface-top);
}
.about-description {
  margin: 0;
  color: var(--text-primary);
  font-size: 1.05rem;
  line-height: 1.75;
  white-space: pre-wrap;
}
.about-description.muted {
  color: var(--text-secondary);
}
.about-meta {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 1rem;
  margin: 0;
}
.about-meta div {
  display: grid;
  gap: 0.35rem;
  padding-top: 1rem;
  border-top: var(--border-default);
}
.about-meta dt {
  color: var(--text-secondary);
  font-size: 0.78rem;
  font-weight: 700;
  text-transform: uppercase;
}
.about-meta dd {
  margin: 0;
  color: var(--text-primary);
  font-size: 0.9rem;
}
@media (width <= 640px) {
  .about-meta {
    grid-template-columns: 1fr;
  }
}
</style>
