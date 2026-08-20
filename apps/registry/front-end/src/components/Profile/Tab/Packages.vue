<!-- @/components/Profile/Tab/Packages.vue -->
<template>
  <StateDisplay
    v-if="!isSuccess"
    size="small"
    :type="status"
    :error-message="error?.message"
  />

  <EmptyCard
    v-else-if="isEmpty"
    :title="t('package.empty.title')"
    :message="t('package.empty.desc')"
  />

  <ul v-else class="content-layout clients-list">
    <OptionItemCard
      v-for="item in packages"
      :key="item.id"
      :show-icon="false"
      :item-name="item.fullName"
      :created-at="item.createdAt"
      :item-status="item.description"
      @click="handleCardClick(item.fullName)"
    />
  </ul>
</template>

<script setup lang="ts">
import EmptyCard from '@/components/Option/Card/Empty.vue';
import OptionItemCard from '@/components/Option/Card/ItemCard.vue';

import { toRef } from 'vue';
import { useLocale } from '@fuyeor/locale';
import { useRouter } from '@fuyeor/vue-router';
import { StateDisplay } from '@fuyeor/interactify';
import { useUserPackages } from '@/composables/api/useUsers';
import type { User } from '@/types/user';

const props = defineProps<{
  user: User;
}>();

const { t } = useLocale();
const router = useRouter();

const usernameRef = toRef(props.user, 'username');

const {
  data: packages,
  status,
  error,
  isSuccess,
  isEmpty,
} = useUserPackages(usernameRef);

/**
 * go to package - en/package/@scope/name
 */
const handleCardClick = (fullName: string) => {
  // fullName, e.g. "@fuyeor/std"
  router.push({
    name: 'Package',
    params: {
      scope: fullName.split('/')[0], // @fuyeor
      name: fullName.split('/')[1], // std
    },
  });
};
</script>
