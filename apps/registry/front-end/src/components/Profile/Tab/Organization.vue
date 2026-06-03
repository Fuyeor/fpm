<!-- @/components/Profile/Tab/Organization.vue -->
<template>
  <div class="organization-tab-container">
    <a
      v-if="isCurrentUser"
      class="tip-bar about-edit-tip clickable"
      @click.prevent="openAddModal"
    >
      <img class="tip-icon" :src="getIconUrl('edit')" alt="" />
      <small> {{ t('organization.add') }} </small>
    </a>

    <StateDisplay
      v-if="!isSuccess"
      :type="status"
      :error-message="error?.message"
    />

    <EmptyCard v-else-if="isEmpty" />

    <ul v-else class="clients-list">
      <OptionItemCard
        v-for="item in organizations"
        :key="item.id"
        icon-text="Scope"
        :item-name="'@' + item.name"
        :item-status="t('role.' + item.role)"
      />
    </ul>
  </div>

  <CreateOrganizationModal v-model="showModal" />
</template>

<script setup lang="ts">
import CreateOrganizationModal from '../Modal/CreateOrganizationModal.vue';

import { ref, toRef } from 'vue';
import { useLocale } from '@fuyeor/locale';
import { getIconUrl } from '@fuyeor/commons';
import { StateDisplay, OptionItemCard } from '@fuyeor/interactify';
import { useAuth } from '@/composables/auth/useAuth';
import { useUserOrganizations } from '@/composables/api/useUsers';
import type { User } from '@/types/user';

const props = defineProps<{
  user: User;
}>();

const { t } = useLocale();
const { checkIsCurrentUser } = useAuth();

const usernameRef = toRef(props.user, 'username');

const {
  data: organizations,
  status,
  error,
  isSuccess,
  isEmpty,
} = useUserOrganizations(usernameRef);

const isCurrentUser = checkIsCurrentUser(props.user.id);
const showModal = ref(false);

const openAddModal = () => {
  showModal.value = true;
};
</script>
