<!-- @/components/Profile/Tab/Packages.vue -->
<template>
</template>

<script setup lang="ts">
import { useLocale } from '@fuyeor/locale';
import { getIconUrl } from '@fuyeor/commons';
import type { User } from '@/types/user';

const props = defineProps<{
  user: User;
}>();

const { t } = useLocale();
</script>