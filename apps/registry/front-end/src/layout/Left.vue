<!-- @/layout/Left.vue -->
<template>
  <LeftSidebar>
    <template #nav>
      <!-- 未登录状态 -->
      <SidebarMenu v-if="!isAuthenticated" :items="signedOutNavItems" />

      <!-- 登录状态 -->
      <template v-else>
        <!-- 菜单导航 -->
        <Foldable
          :foldable="layoutStore.shouldShow"
          :modelValue="!layoutStore.shouldShow"
          :title="t('workspace')"
          :icon-url="getIconUrl('home')"
        >
          <SidebarMenu :items="signnedInNavItems" />
        </Foldable>

        <div id="left-sidebar-anchor"></div>
      </template>
    </template>

    <template v-if="!isMobile && currentUser" #footer>
      <SidebarUserCard
        :avatar="getAvatarUrl(currentUser.avatar)"
        :nickname="currentUser.nickname"
        :username="currentUser.username"
      />
    </template>
  </LeftSidebar>
</template>

<script setup lang="ts">
import { useLocale } from '@fuyeor/locale';
import { getIconUrl, getAvatarUrl } from '@fuyeor/commons';
import { useMobileDetection, useSidebarItems } from '@fuyeor/interactify';
import {
  LeftSidebar,
  Foldable,
  SidebarMenu,
  SidebarUserCard,
  type MenuItem,
} from '@fuyeor/interactify';
import { useLayoutStore } from '@/stores/layout';
import { useAuth } from '@/composables/auth/useAuth';
import { useMenuConfig } from '@/config/sidebar/menu.config';

const { t } = useLocale();
const { isMobile } = useMobileDetection();
const { isAuthenticated, currentUser } = useAuth();
const { signedOutNavItemsRaw, signedInNavItemsRaw } = useMenuConfig();

const layoutStore = useLayoutStore();

// 生成最终的导航项
const { processedItems: signedOutNavItems } = useSidebarItems(
  signedOutNavItemsRaw,
  { t },
);
const { processedItems: signnedInNavItems } = useSidebarItems(
  signedInNavItemsRaw,
  { t },
);
</script>
