<!-- @/components/Option/Menu.vue -->
<template>
  <div class="settings-menu">
    <!--<h2 class="menu-header">{{ t(menu.title) }}</h2>-->
    <p v-if="menu.description" class="menu-description">{{ t(menu.description) }}</p>

    <!-- 遍历菜单组 -->
    <template v-for="(group, groupIndex) in menu.groups" :key="groupIndex">
      <h3 class="menu-h3" v-if="group.title" :id="group.id">{{ t(group.title) }}</h3>

      <!-- 遍历组内的菜单项 -->
      <template v-for="item in group.items" :key="item.to">
        <!-- 优化：使用 isItemVisible 函数进行判断，保持模板干净 -->
        <OptionItem
          v-if="isItemVisible(item)"
          :icon="item.icon"
          :title="t(item.title)"
          :description="t(item.description)"
          :to="item.to"
        />
      </template>
    </template>
  </div>
</template>

<script setup lang="ts">
import { useLocale } from '@fuyeor/locale';
import { OptionItem } from '@fuyeor/interactify';
import { useAuth } from '@/composables/auth/useAuth';
import type { OptionMenu, MenuItem } from '@/types/options/options-menu';

// 接收菜单配置对象
defineProps<{
  menu: OptionMenu;
}>();

const { t } = useLocale();
const { currentUser } = useAuth();

// 权限判断逻辑函数
const isItemVisible = (item: MenuItem): boolean => {
  // 如果菜单项没有定义 requiredType，则对所有人可见
  if (!item.requiredType || item.requiredType.length === 0) {
    return true;
  }

  // 从 userStore.currentUser 获取用户角色
  const userType = currentUser.value?.type;

  // 如果用户角色存在，并且在 requiredType 数组中，则可见
  if (userType && item.requiredType.includes(userType)) {
    return true;
  }

  // 其他所有情况都不可见
  return false;
};
</script>
