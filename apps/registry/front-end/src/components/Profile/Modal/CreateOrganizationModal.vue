<!-- @/components/Profile/Modal/CreateOrganizationModal.vue -->
<template>
  <Modal
    :model-value="modelValue"
    @update:model-value="emit('update:modelValue', $event)"
    strict
  >
    <template #header>
      <h3>{{ t('organization.add') }}</h3>
    </template>

    <FormField
      :label="t('organization.name')"
      :hint="validationHint"
      :error="validationErrorMsg || creationErrorMsg"
      :tip="t('username.hint')"
    >
      <input
        type="text"
        v-model="scopeName"
        @input="handleInput"
        :disabled="isCreating"
      />
      <span v-if="isValidating" class="spinner-icon">⏳</span>
    </FormField>

    <template #footer>
      <div class="modal-actions">
        <button
          class="btn-secondary"
          @click="closeModal"
          :disabled="isCreating"
        >
          {{ t('cancel') }}
        </button>
        <button
          class="btn-primary"
          @click="handleCreate"
          :disabled="!isAvailable || isCreating || isValidating"
        >
          {{ isCreating ? t('creating') : t('create') }}
        </button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useLocale } from '@fuyeor/locale';
import { Modal, FormField } from '@fuyeor/interactify';
import { useOrganizations } from '@/composables/api/useOrganizations';

defineProps<{
  modelValue: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
}>();

const { t } = useLocale();
const {
  validateScope,
  isValidating,
  validationData,
  createOrganization,
  isCreating,
  creationError,
} = useOrganizations();

const scopeName = ref('');
let debounceTimer: any = null;

const isAvailable = computed(() => !!validationData.value?.available);

const validationHint = computed(() => {
  if (!scopeName.value) return t('organization.name.hint');
  if (isValidating.value) return '';
  if (isAvailable.value) return t('organization.name.available');
  return '';
});

const validationErrorMsg = computed(() => {
  if (scopeName.value && !isValidating.value && !isAvailable.value) {
    return t(validationData.value?.message || 'scope.invalid');
  }
  return '';
});

const creationErrorMsg = computed(() => {
  if (creationError.value) return t('organization.create.failed');
  return '';
});

const closeModal = () => {
  emit('update:modelValue', false);
};

const handleInput = () => {
  if (debounceTimer) clearTimeout(debounceTimer);
  const username = scopeName.value.trim();
  if (!username) return;

  debounceTimer = setTimeout(() => {
    validateScope({ username });
  }, 400);
};

const handleCreate = async () => {
  if (!isAvailable.value || isCreating.value) return;
  try {
    await createOrganization({ username: scopeName.value.trim() });
    closeModal();
  } catch (err) {
    console.error('[Organization] Create failed:', err);
  }
};
</script>

<style scoped>
.spinner-icon {
  position: absolute;
  right: 10px;
  animation: spin 1s linear infinite;
}
@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
