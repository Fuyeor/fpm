<!-- @/views/Options/Index.vue -->
<template>
  <!-- 动态生成的仪表盘网格 -->
  <div v-if="isDashboard" class="option-hub-layout">
    <HeaderBar :title="t('settings')" />

    <option-menu-component :menu="dashboardMenu" />
  </div>

  <!-- 当进入子页面时，显示页面内容 -->
  <div v-else class="option-hub-layout">
    <keep-alive :max="5" :key="route.path">
      <router-view />
    </keep-alive>
  </div>
</template>

<script setup lang="ts">
import OptionMenuComponent from '@/components/Option/Menu.vue';

import { computed } from 'vue';
import { useRoute } from '@fuyeor/vue-router';
import { useLocale } from '@fuyeor/locale';
import { HeaderBar } from '@fuyeor/interactify';
import { dashboardMenu } from '@/config/options/dashboard.menu';

const route = useRoute();
const { t } = useLocale();

// 判断当前是否在仪表盘主页
const isDashboard = computed(() => route.name === 'Option');
</script>
