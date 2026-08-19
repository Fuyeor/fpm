<!-- @/components/Option/Card/ItemCard.vue -->
<template>
  <div class="item-card" @click="handleClick">
    <img
      v-if="showIcon"
      class="item-logo"
      v-tooltip="{ text: iconText, placement: 'top' }"
      :src="iconUrl || getIconUrl('earth')"
    />
    <div class="item-info">
      <h3 class="item-name">{{ itemName }}</h3>
      <p v-if="itemDesc" class="item-id">{{ itemDesc }}</p>
      <p class="item-meta">
        {{ t('created.at') }} / {{ formatDate(createdAt, { preset: 'full' }) }}
      </p>
    </div>
    <div v-if="itemStatus" class="item-status" :class="`status-${itemStatus}`">
      {{ itemStatus }}
    </div>

    <div v-if="$slots.menu" class="item-menu">
      <slot name="menu"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useLocale } from '@fuyeor/locale';
import { getIconUrl, useDateFormatter } from '@fuyeor/commons';

withDefaults(
  defineProps<{
    showIcon?: boolean;
    // The URI for the item's logo.
    iconUrl?: string;
    // The descriptive text for the item's logo.
    iconText?: string;
    // The name of the item application.
    itemName: string;
    // The item ID string.
    itemDesc?: string;
    // The item's creation date string.
    createdAt: string;
    // The status of the item.
    itemStatus?: string;
  }>(),
  {
    showIcon: true,
  },
);

/**
 * Corresponds to the user's 'click' prop request.
 * Emits a 'click' event when the card is clicked.
 */
const emit = defineEmits(['click']);
const { t } = useLocale();
const { formatDate } = useDateFormatter();

const handleClick = () => {
  emit('click');
};
</script>

<style scoped>
.item-card {
  display: flex;
  align-items: center;
  padding: 1.5rem;
  border: var(--border-default);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.3s;
}
.item-card:hover {
  border: var(--border-subtle);
  background-color: var(--surface-raised-hover);
}
.item-logo {
  width: 50px;
  height: 50px;
  border-radius: var(--radius-sm);
  margin-right: 1.5rem;
  object-fit: cover;
  background: var(--surface);
}
.item-info {
  flex-grow: 1;
  display: flex;
  gap: 10px;
  flex-direction: column;
}
.item-name {
  font-size: 1.2rem;
  font-weight: 600;
  margin: 0 0 0.25rem 0;
}
.item-id,
.item-meta {
  font-size: 0.9rem;
  margin: 0;
  word-break: break-all;
  /* 多行省略 */
  display: -webkit-box;
  line-clamp: 2;
  overflow: hidden;
  text-overflow: ellipsis;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}
.item-status {
  margin-right: 1rem;
  padding: 0.25rem 0.75rem;
  border-radius: 1rem;
  font-size: 0.8rem;
  font-weight: bold;
  text-transform: capitalize;
}
.status-active {
  background: #e6f7ec;
  color: #008744;
}
.status-suspended,
.status-invalid,
.status-disabled {
  background: #fff1f0;
  color: #d93026;
}
@media (width <= 768px) {
  .item-card {
    padding: 1.5rem 1rem;
  }
  .item-logo {
    width: 36px;
    height: 36px;
    margin-right: 0.75rem;
  }
  .item-menu {
    margin-left: 0.75rem;
  }
}
</style>