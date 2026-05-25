// @/stores/layout.ts
import { ref, computed } from 'vue';
import { defineStore } from '@fuyeor/commons';

export const useLayoutStore = defineStore('layout', () => {
  const anchorCount = ref(0);

  const shouldShow = computed(() => anchorCount.value > 0);

  return {
    shouldShow,
    add: () => {
      anchorCount.value++;
    },
    del: () => {
      anchorCount.value = Math.max(0, anchorCount.value - 1);
    },
  };
});
