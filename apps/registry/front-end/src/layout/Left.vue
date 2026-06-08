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
          <SidebarMenu :items="signedInNavItems" />
        </Foldable>

        <!-- organizations -->
        <Foldable
          :foldable="true"
          :modelValue="!layoutStore.shouldShow"
          :title="t('organizations')"
          :icon-url="getIconUrl('company')"
        >
          <SidebarMenu :items="orgNavItems" />
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
import { computed } from 'vue';
import { useRoute } from '@fuyeor/vue-router';
import { useLocale } from '@fuyeor/locale';
import { getIconUrl, getAvatarUrl } from '@fuyeor/commons';
import { useMobileDetection, useSidebarItems } from '@fuyeor/interactify';
import {
  LeftSidebar,
  Foldable,
  SidebarMenu,
  SidebarUserCard,
} from '@fuyeor/interactify';
import { useLayoutStore } from '@/stores/layout';
import { useAuth } from '@/composables/auth/useAuth';
import { useUserStore } from '@/stores/user';
import { useMenuConfig } from '@/config/sidebar/menu.config';
import type { SidebarItem } from '@fuyeor/interactify';

const { t } = useLocale();
const { isMobile } = useMobileDetection();
const { isAuthenticated, currentUser } = useAuth();
const { signedOutNavItemsRaw, signedInNavItemsRaw } = useMenuConfig();

const route = useRoute();
const layoutStore = useLayoutStore();
const userStore = useUserStore();

// 生成主导航项
const { processedItems: signedOutNavItems } = useSidebarItems(
  signedOutNavItemsRaw,
  { t },
);
const { processedItems: signedInNavItems } = useSidebarItems(
  signedInNavItemsRaw,
  { t },
);

const orgNavItems = computed<SidebarItem[]>(() => {
  const locale = route.params.locale;

  return (userStore.organizations || []).map((org) => ({
    type: 'link',
    tag: 'router-link',
    linkProps: {
      to: `/organization/@${org.username}`,
    },
    icon: getIconUrl('email'),
    text: `@${org.username}`,
  }));
});
</script>
