// @/stores/config.ts
import { ref, computed } from 'vue';
import { defineStore } from '@fuyeor/commons';

export const useConfigStore = defineStore('config', () => {
  const name = ref('site.name');
  const title = ref('site.title');
  const url = ref('https://fpm.fuyeor.com');
  const description = computed(() => `${name.value}: ${title.value}`);

  return {
    name,
    title,
    url,
    description,
  };
});
