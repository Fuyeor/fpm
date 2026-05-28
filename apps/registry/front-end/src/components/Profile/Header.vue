<!-- @/components/Profile/Header.vue -->
<template>
  <div class="profile-header">
    <div class="profile-cover"></div>
    <div class="profile-avatar-wrapper">
      <div
        class="profile-avatar"
        @click="hasCustomAvatar ? viewAvatar() : null"
      >
        <!-- 头像图片本身 -->
        <img :src="resolvedAvatar" alt="" fetchpriority="high" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { getAvatarUrl } from '@fuyeor/commons';
import { useImageViewer, type ImageViewerItem } from '@fuyeor/interactify';
import type { User } from '@/types/user';

const props = defineProps<{
  user: User;
}>();

const { open: openImageViewer } = useImageViewer();

// 判断用户是否有自定义头像
const hasCustomAvatar = computed(() => {
  // 只要 user 对象存在，并且 avatar 字段不是 null/undefined/空字符串，就认为有
  return !!props.user.avatar;
});

// 解析头像和封面图 URL
const resolvedAvatar = computed(() => getAvatarUrl(props.user.avatar));

const viewAvatar = () => {
  // 确保用户数据存在
  if (!props.user) return;

  // 准备要预览的图片数据
  const items: ImageViewerItem[] = [
    {
      src: resolvedAvatar.value,
      w: 1024,
      h: 1024,
      alt: props.user.nickname,
    },
  ];

  // 打开预览
  openImageViewer(items);
};
</script>

<style>
.profile-header {
  position: relative;
  /* 设置相对定位 */
}

.profile-cover {
  position: relative;
  overflow: hidden;
  width: 100%;
  height: 300px;
  transform: translateZ(0);
  background: var(--surface-top);
}

.profile-cover img {
  /* 让图片撑满父容器 */
  width: 100%;
  height: 100%;
  object-fit: cover;
  position: relative;
  /* 确保图片渲染质量 */
  image-rendering: auto;
  image-rendering: -webkit-optimize-contrast;
  z-index: 0;
  /* 视差效果性能优化 */
  transform: translateZ(0);
  will-change: transform;
}

.profile-avatar-wrapper {
  position: absolute;
  bottom: -70px;
  left: 36px;
}

.profile-avatar {
  width: 150px;
  height: 150px;
  border-radius: 10%;
  overflow: hidden;
  border: 5px solid #fff;
  cursor: pointer;
  background: var(--surface-hover);
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
}

.profile-avatar img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}
</style>
