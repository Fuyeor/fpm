<!-- apps/registry/front-end/src/components/Shared/CopyClipboard.vue -->
<template>
  <div class="copy-clipboard">
    <code>{{ value }}</code>
    <p v-if="description">{{ description }}</p>
    <button type="button" @click="copyValue">
      {{ copied ? 'Copied' : 'Copy' }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps<{
  value: string;
  description?: string;
}>();

const copied = ref(false);

const copyValue = async () => {
  await window.navigator.clipboard.writeText(props.value);
  copied.value = true;
};
</script>
