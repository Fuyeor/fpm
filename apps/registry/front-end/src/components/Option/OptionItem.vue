<!-- apps/registry/front-end/src/components/Option/OptionItem.vue -->
<template>
  <a class="option-item" href="#" @click.prevent="navigate">
    <img v-if="icon" class="option-item-icon" :src="icon" alt="" />
    <span class="option-item-content">
      <strong>{{ title }}</strong>
      <small v-if="description">{{ description }}</small>
    </span>
  </a>
</template>

<script setup lang="ts">
import { useRouter } from '@fuyeor/vue-router';
import type { RouteLocationRaw } from '@fuyeor/vue-router';

const props = defineProps<{
  icon?: string;
  title: string;
  description?: string;
  to: RouteLocationRaw;
}>();

const router = useRouter();

// Navigates through the registry app router without requiring a missing RouterLink export.
const navigate = () => router.push(props.to);
</script>
