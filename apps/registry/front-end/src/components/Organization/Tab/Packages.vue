<!-- @/components/Organization/Tab/Packages.vue -->
<template>
  <section class="organization-tab" aria-labelledby="organization-packages-title">
    <header class="tab-heading">
      <div>
        <p class="eyebrow">{{ t('packages') }}</p>
        <h2 id="organization-packages-title">{{ t('organization.packages.desc') }}</h2>
      </div>
      <span v-if="isRetrieved" class="count-badge">{{ packages.length }}</span>
    </header>

    <StateDisplay
      v-if="!isRetrieved"
      :type="status"
      :error-message="error?.message"
    />
    <EmptyCard v-else-if="isEmpty" />
    <div v-else class="package-list">
      <OptionItemCard
        v-for="item in packages"
        :key="item.id"
        :show-icon="false"
        :item-name="item.fullName"
        :item-desc="item.description"
        :created-at="item.createdAt"
        @click="openPackage(item.fullName)"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
import { toRef } from 'vue';
import { useLocale } from '@fuyeor/locale';
import { useRouter } from '@fuyeor/vue-router';
import { StateDisplay } from '@fuyeor/interactify';
import EmptyCard from '@/components/Option/Card/Empty.vue';
import OptionItemCard from '@/components/Option/Card/ItemCard.vue';
import { useOrganizationPackages } from '@/composables/api/useOrganizationsPublic';

const props = defineProps<{
  username: string;
}>();

const { t } = useLocale();
const router = useRouter();
const usernameRef = toRef(props, 'username');
const { data: packages, status, error, isRetrieved, isEmpty } =
  useOrganizationPackages(usernameRef);

const openPackage = (fullName: string) => {
  const [scope, name] = fullName.split('/');
  if (!scope || !name) return;
  router.push({ name: 'Package', params: { scope, name } });
};
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
.package-list {
  display: grid;
  gap: 0.75rem;
}
</style>
