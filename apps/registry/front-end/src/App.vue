<!-- @/App.vue -->
<template>
  <left-sidebar />

  <main>
    <router-view />
  </main>

  <!-- 根据是否为移动端来条件渲染右侧边栏 -->
  <right-sidebar v-if="!isMobile" />

  <!-- 根据条件渲染底部导航栏 -->
  <bottom-sidebar v-if="shouldShowBottomSidebar" />

  <back-top />

  <!-- toast notifications -->
  <toast-provider position="bottom-right" />
</template>

<script setup lang="ts">
import LeftSidebar from '@/layout/Left.vue';

import { computed } from 'vue';
import { useHeadManager } from '@fuyeor/commons';
import {
  useFontLoader,
  useMobileDetection,
  BackTop,
  ToastProvider,
} from '@fuyeor/interactify';
import { useAsyncComponents } from '@/composables/loader/useAsyncComponents';

const { isMobile } = useMobileDetection();

// get all async components
const { RightSidebar, BottomSidebar } = useAsyncComponents();

// show when mobile
const shouldShowBottomSidebar = computed(() => {
  return isMobile.value;
});

// 启用字体加载
useFontLoader();
// 启动全局标题管理器
useHeadManager({
  nameKey: 'site.name',
  titleKey: 'site.title',
});
</script>
