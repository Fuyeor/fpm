<!-- apps/registry/front-end/src/components/Option/OptionItemCard.vue -->
<template>
  <li class="option-item-card" @click="$emit('click')">
    <span v-if="iconText" class="option-item-card-icon">{{ iconText }}</span>
    <span class="option-item-card-content">
      <strong>{{ itemName }}</strong>
      <small v-if="createdAt">{{ createdAt }}</small>
    </span>
    <span v-if="itemStatus" class="option-item-card-status">{{ itemStatus }}</span>
    <span class="option-item-card-actions" @click.stop>
      <slot />
    </span>
  </li>
</template>

<script setup lang="ts">
defineProps<{
  iconText?: string;
  itemName: string;
  createdAt?: string;
  itemStatus?: string;
}>();

defineEmits<{
  click: [];
}>();
</script>
