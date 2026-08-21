<!-- @/components/Organization/Header.vue -->
<template>
  <section class="organization-header" aria-labelledby="organization-name">
    <div class="organization-cover">
      <div class="cover-orbit cover-orbit-large"></div>
      <div class="cover-orbit cover-orbit-small"></div>
    </div>
    <div class="organization-identity">
      <div class="organization-mark" aria-hidden="true">{{ initial }}</div>
      <div class="organization-heading">
        <p class="organization-kicker">Organization</p>
        <h1 id="organization-name">@{{ organization.username }}</h1>
        <p v-if="organization.description" class="organization-description">
          {{ organization.description }}
        </p>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { OrganizationProfile } from '@/types/organization';

const props = defineProps<{
  organization: OrganizationProfile;
}>();

const initial = computed(() => props.organization.username.slice(0, 1).toUpperCase());
</script>

<style scoped>
.organization-header {
  overflow: hidden;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--surface-top);
}
.organization-cover {
  position: relative;
  height: 184px;
  overflow: hidden;
  background: linear-gradient(118deg, #172554 0%, #2563eb 47%, #06b6d4 100%);
}
.organization-cover::after {
  position: absolute;
  inset: 0;
  background: linear-gradient(90deg, rgb(15 23 42 / 40%), transparent 60%);
  content: '';
}
.cover-orbit {
  position: absolute;
  border: 1px solid rgb(255 255 255 / 35%);
  border-radius: 999px;
  transform: rotate(-18deg);
}
.cover-orbit-large {
  top: -155px;
  right: -54px;
  width: 520px;
  height: 360px;
}
.cover-orbit-small {
  top: -102px;
  right: 60px;
  width: 310px;
  height: 220px;
  opacity: 0.65;
}
.organization-identity {
  display: flex;
  gap: 1.5rem;
  align-items: flex-end;
  padding: 0 2rem 2rem;
}
.organization-mark {
  display: grid;
  flex: 0 0 auto;
  width: 112px;
  height: 112px;
  margin-top: -56px;
  place-items: center;
  border: 5px solid var(--surface-top);
  border-radius: 28px;
  background: linear-gradient(135deg, #0f172a, #334155);
  box-shadow: 0 16px 32px rgb(15 23 42 / 20%);
  color: #fff;
  font-size: 2.5rem;
  font-weight: 700;
  letter-spacing: -0.08em;
}
.organization-heading {
  min-width: 0;
  padding-top: 1.25rem;
}
.organization-kicker {
  margin: 0 0 0.4rem;
  color: var(--text-secondary);
  font-size: 0.78rem;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}
.organization-heading h1 {
  margin: 0;
  color: var(--text-primary);
  font-size: clamp(1.8rem, 4vw, 2.6rem);
  letter-spacing: -0.04em;
}
.organization-description {
  max-width: 720px;
  margin: 0.7rem 0 0;
  color: var(--text-secondary);
  line-height: 1.6;
}
@media (width <= 640px) {
  .organization-cover {
    height: 140px;
  }
  .organization-identity {
    display: block;
    padding: 0 1rem 1.5rem;
  }
  .organization-mark {
    width: 88px;
    height: 88px;
    margin-top: -44px;
    border-radius: 22px;
    font-size: 2rem;
  }
  .organization-heading {
    padding-top: 1rem;
  }
}
</style>
