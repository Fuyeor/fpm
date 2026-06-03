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
    >
      <input
        type="text"
        v-model="scopeName"
        @input="handleInput"
        :placeholder="t('organization.name.placeholder')"
        :disabled="isCreating"
        class="fpm-input"
      />
      <span v-if="isValidating" class="spinner-icon">⏳</span>
    </FormField>

    <template #footer>
      <div class="modal-footer">
        <button class="btn-cancel" @click="closeModal" :disabled="isCreating">
          {{ t('cancel') }}
        </button>
        <button
          class="btn-submit"
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
  const name = scopeName.value.trim();
  if (!name) return;

  debounceTimer = setTimeout(() => {
    validateScope({ name });
  }, 400);
};

const handleCreate = async () => {
  if (!isAvailable.value || isCreating.value) return;
  try {
    await createOrganization({ name: scopeName.value.trim() });
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
.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 1rem;
}
.btn-submit {
  background: #007aff;
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  cursor: pointer;
}
.btn-submit:disabled {
  background: #ccc;
  cursor: not-allowed;
}
.btn-cancel {
  background: transparent;
  border: 1px solid #ccc;
  padding: 8px 16px;
  border-radius: 6px;
  cursor: pointer;
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
