<!-- @/components/Option/Card/Empty.vue -->
<template>
  <div class="empty-state">
    <!-- 标题插槽（默认用 title prop） -->
    <p v-if="!$slots.title" class="title">{{ title ?? t('content.empty') }}</p>
    <slot name="title" />

    <!-- 描述插槽（默认用 description prop） -->
    <p v-if="!$slots.default">
      {{ description }}
    </p>
    <slot />
  </div>
</template>

<script setup lang="ts">
import { useLocale } from '@fuyeor/locale';

defineProps<{
  title?: string;
  description?: string;
}>();

const { t } = useLocale();
</script>

<style scoped>
.empty-state {
  padding: 4rem 2rem;
  background: var(--surface-top);
  border-radius: var(--radius-md);

  .title {
    font-size: 1.1rem;
    font-weight: 600;
  }
}
</style>