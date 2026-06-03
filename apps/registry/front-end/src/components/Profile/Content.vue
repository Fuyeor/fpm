<!-- @/components/Profile/Content.vue -->
<template>
  <!-- 内容选项卡 或 搜索框 -->
  <div class="tabs-or-search-container">
    <!-- 使用 transition 实现平滑切换效果 -->
    <transition name="fade" mode="out-in">
      <!-- 条件渲染：如果不在搜索模式，显示 Tabs -->
      <Tabs
        v-if="!isSearchBarVisible"
        key="tabs"
        containerClass="profile"
        :tabs="profilePageOptions"
        :active-tab-value="currentTab"
        @tab-click="handleTabClickInProfile"
      >
        <!-- 搜索图标 -->
        <template #search-trigger>
          <img
            v-tooltip="{
              text: t('search.content.with', { username: username }),
              placement: 'top',
            }"
            :src="getIconUrl('search')"
          />
        </template>
      </Tabs>
      <!-- 否则，显示我们全新的搜索框 -->
      <UserSearchBar
        v-else
        key="search-bar"
        :username="username"
        @search="handleExecuteSearch"
        @close="handleCloseSearch"
      />
    </transition>
  </div>

  <!-- 根据 currentTab 渲染不同内容 -->
  <div
    class="profile-content-area"
    :class="`tab-${currentTab}`"
    :key="isSearching ? 'search-content' : 'tab-content'"
  >
    <!-- 如果不在搜索模式，显示原来的 Tab 内容 -->
    <Suspense v-if="!isSearching">
      <template #default>
        <!-- 使用 KeepAlive 缓存 Tab 页面 -->
        <KeepAlive :max="5">
          <component
            :is="getComponentForTab(currentTab)"
            :key="route.path"
            :user="user"
            :username="username"
          />
        </KeepAlive>
      </template>
      <template #fallback>
        <StateDisplay type="loading" />
      </template>
    </Suspense>

    <!-- 如果在搜索模式，显示搜索结果 -->
    <template v-else>
      <Tabs
        container-class="search"
        :tabs="searchTabs"
        :active-tab-value="currentSearchTab"
        :is-router-nav="false"
        @tab-click="handleSearchTabChange"
      />
      <!-- 用动态组件直接渲染，并传递 searchQuery -->
      <component
        v-if="route.query.q"
        :is="getComponentForSearchTab(currentSearchTab)"
        :key="`${currentSearchTab}-${route.query.q}`"
        :username="username"
        :searchQuery="route.query.q"
      />
    </template>
  </div>
</template>

<script setup lang="ts">
const UserPackages = defineAsyncComponent(
  () => import('@/components/Profile/Tab/Packages.vue'),
);
const UserOrganizations = defineAsyncComponent(
  () => import('@/components/Profile/Tab/Organization.vue'),
);

import { ref, computed, defineAsyncComponent, watch } from 'vue';
import { useLocale } from '@fuyeor/locale';
import { useRoute, useRouter } from '@fuyeor/vue-router';
import { getIconUrl } from '@fuyeor/commons';
import { StateDisplay, Tabs, type TabItem } from '@fuyeor/interactify';
import { useAuth } from '@/composables/auth/useAuth';
import { useLocalePath } from '@/composables/useLocalePath';
import type { User } from '@/types/user';

// 定义 Props
const props = defineProps<{
  tab?: string;
  user: User;
  username: string;
}>();

const route = useRoute();
const router = useRouter();
const localePath = useLocalePath();

const { t } = useLocale();
const { isAuthenticated } = useAuth();

// 用户内容相关的状态
const currentTab = ref(props.tab || 'activities');

// 搜索相关的状态
const _isSearchBarVisible = ref(false);
const searchEntryPath = ref<string>(''); // 进入搜索前的位置
// 通过 URL 查询参数判断是否处于搜索模式
const isSearching = computed(() => route.path.endsWith('/search'));
const currentSearchTab = ref('all'); // 管理搜索结果的 Tab

const isSearchBarVisible = computed({
  // get：当 URL 是搜索模式，或者我们手动打开了搜索框时，都应该显示
  get: () => isSearching.value || _isSearchBarVisible.value,
  // set：我们仍然可以手动设置它 (主要用于关闭)
  set: (val) => {
    _isSearchBarVisible.value = val;
  },
});

const profilePageOptions = computed<TabItem[]>(() => {
  const basePath = localePath(`/@${props.username}`);

  const base: TabItem[] = [
    { labelKey: 'activities', value: 'activities', path: basePath },
    { labelKey: 'packages', value: 'packages', path: `${basePath}/packages` },
    {
      labelKey: 'organizations',
      value: 'organizations',
      path: `${basePath}/organizations`,
    },

    { labelKey: 'about', value: 'about', path: `${basePath}/about` },
  ];

  // 搜索只对登录用户显示
  if (isAuthenticated.value) {
    // 把搜索图标 Tab 加到数组末尾
    base.push({ value: 'search-trigger', class: 'search-tab-item' });
  }

  return base;
});

const getComponentForTab = (tabValue: string): any => {
  switch (tabValue) {
    case 'packages':
      return UserPackages;
    case 'organizations':
      return UserOrganizations;
    case 'about':
      return;
    default:
      return;
  }
};

// 定义搜索结果页的 Tabs
const searchTabs = computed<TabItem[]>(() => [
  { labelKey: 'all', value: 'all' },
  { labelKey: 'thoughts', value: 'thought' },
  { labelKey: 'comments', value: 'comment' },
]);

const getComponentForSearchTab = (tabValue: string): any => {
  switch (tabValue) {
    case 'all':
      return;
    case 'thought':
      return;
    case 'comment':
      return;
  }
};

// 关闭搜索框：直接回到我们记住的位置
const handleCloseSearch = () => {
  _isSearchBarVisible.value = false;
  // 如果当前已经在搜索页，点击关闭应该返回到搜索前的位置
  if (isSearching.value) {
    if (searchEntryPath.value) {
      router.push(searchEntryPath.value);
    } else {
      // 提供一个备用返回路径
      router.push(localePath(`/@${props.username}`));
    }
  }
};

// 执行搜索：更新 URL 的 query 参数
const handleExecuteSearch = (query: string) => {
  // 只有在非搜索模式下第一次搜索时，才记录入口路径
  if (!isSearching.value) {
    searchEntryPath.value = route.fullPath;
  }
  _isSearchBarVisible.value = true; // 保持搜索框可见

  const newQuery: { q: string; type?: string } = { q: query };

  // 检查当前路由中是否已经存在 type，如果有，就保留它
  if (route.query.type) {
    newQuery.type = route.query.type as string;
  }

  router.push({
    path: localePath(`/@${props.username}/search`),
    query: newQuery,
  });
};

const handleTabClickInProfile = (payload: TabItem | string) => {
  // 我们只关心 value 是不是 'search-trigger'
  const value = typeof payload === 'string' ? payload : payload.value;

  if (value === 'search-trigger') {
    _isSearchBarVisible.value = true;
    // 这里不需要 return，因为 isRouterNav=true 会阻止后续不必要的导航
  }
};

// 处理搜索结果 Tab 切换的事件
const handleSearchTabChange = (tabValue: string) => {
  currentSearchTab.value = tabValue;
  // 更新 URL 以反映新的 tab 类型
  router.push({
    path: localePath(`/@${props.username}/search`),
    query: { ...route.query, type: tabValue || undefined },
  });
};

// 单独监听 tab 的变化
watch(
  () => props.tab,
  (newTab) => {
    currentTab.value = newTab || 'activities';
  },
);

// 负责所有和搜索相关的逻辑
watch(
  () => [props.tab, route.path, route.query.type],
  ([rawNewTab, newPath, newType]) => {
    // 为了清晰，将 newTab 重命名为 rawNewTab
    // 安全守卫，确保 newPath 是字符串
    if (typeof newPath !== 'string') return;

    if (!newPath.endsWith('/search')) {
      // 只有在非搜索页，才同步 props.tab 到 currentTab

      // 安全地处理 rawNewTab，确保它是字符串
      const newTab = Array.isArray(rawNewTab) ? rawNewTab[0] : rawNewTab;
      currentTab.value = newTab || 'activities';
    }

    // 任何情况下，都同步 URL 的 type 到 currentSearchTab
    const type = Array.isArray(newType) ? newType[0] : newType;
    currentSearchTab.value = type || 'all';
  },
  { immediate: true, deep: true },
);
</script>
