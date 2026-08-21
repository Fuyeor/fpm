<!-- @/components/Organization/Tab/Members.vue -->
<template>
  <section class="organization-tab" aria-labelledby="organization-members-title">
    <header class="tab-heading">
      <div>
        <p class="eyebrow">{{ t('members') }}</p>
        <h2 id="organization-members-title">{{ t('organization.members.desc') }}</h2>
      </div>
      <span v-if="isRetrieved" class="count-badge">{{ members.length }}</span>
    </header>

    <StateDisplay
      v-if="!isRetrieved"
      :type="status"
      :error-message="error?.message"
    />
    <EmptyCard v-else-if="isEmpty" />
    <div v-else class="member-grid">
      <article v-for="member in members" :key="member.id" class="member-card">
        <img
          class="member-avatar"
          :src="getAvatarUrl(member.avatar)"
          :alt="member.nickname"
          loading="lazy"
        />
        <div class="member-copy">
          <h3>{{ member.nickname }}</h3>
          <p>@{{ member.username }}</p>
        </div>
        <span class="member-role">{{ member.role }}</span>
      </article>
    </div>
  </section>
</template>

<script setup lang="ts">
import { toRef } from 'vue';
import { useLocale } from '@fuyeor/locale';
import { getAvatarUrl } from '@fuyeor/commons';
import { StateDisplay } from '@fuyeor/interactify';
import EmptyCard from '@/components/Option/Card/Empty.vue';
import { useOrganizationMembers } from '@/composables/api/useOrganizationsPublic';

const props = defineProps<{
  username: string;
}>();
const { t } = useLocale();
const usernameRef = toRef(props, 'username');
const { data: members, status, error, isRetrieved, isEmpty } =
  useOrganizationMembers(usernameRef);
</script>

<style scoped>
.organization-tab {
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
  max-width: 640px;
  margin: 0;
  color: var(--text-primary);
  font-size: 1.05rem;
  font-weight: 500;
  line-height: 1.5;
}
.count-badge {
  display: inline-grid;
  min-width: 2rem;
  height: 2rem;
  padding: 0 0.5rem;
  place-items: center;
  border-radius: 999px;
  background: var(--surface-raised-hover);
  color: var(--text-secondary);
  font-size: 0.82rem;
  font-weight: 700;
}
.member-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(230px, 1fr));
  gap: 0.75rem;
}
.member-card {
  display: flex;
  min-width: 0;
  align-items: center;
  gap: 0.8rem;
  padding: 1rem;
  border: var(--border-default);
  border-radius: var(--radius-md);
  background: var(--surface-top);
}
.member-avatar {
  width: 44px;
  height: 44px;
  flex: 0 0 auto;
  border-radius: 14px;
  object-fit: cover;
  background: var(--surface-raised-hover);
}
.member-copy {
  min-width: 0;
  flex: 1;
}
.member-copy h3,
.member-copy p {
  overflow: hidden;
  margin: 0;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.member-copy h3 {
  color: var(--text-primary);
  font-size: 0.95rem;
}
.member-copy p {
  margin-top: 0.25rem;
  color: var(--text-secondary);
  font-size: 0.8rem;
}
.member-role {
  padding: 0.25rem 0.5rem;
  border-radius: 999px;
  background: var(--surface-raised-hover);
  color: var(--text-secondary);
  font-size: 0.72rem;
  font-weight: 700;
  text-transform: capitalize;
}
</style>
