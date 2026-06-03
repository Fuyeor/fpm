<!-- @/components/Profile/Info.vue -->
<template>
  <div class="profile-content">
    <!-- 昵称和用户名 -->
    <h1 class="nickname">
      {{ user.nickname }}
    </h1>
    <div class="username">@{{ user.username }}</div>

    <!-- 显示“查看详情”按钮 -->
    <router-link
      class="link expand-more"
      :to="{ name: 'Profile', params: { tab: 'about' } }"
    >
      {{ t('profile.viewAbout') }}
      <img :src="getIconUrl('back')" />
    </router-link>
  </div>
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

<style>
.profile-content {
  max-width: 620px;
  margin: 0 auto;
  margin-top: 100px;

  .nickname {
    font-size: 1.6rem;
    font-weight: bold;
    margin-bottom: 8px;
  }

  .username {
    color: #8e909b;
    font-size: 1rem;
    margin-bottom: 20px;
  }

  /* 徽章 */
  .verified-badge {
    --badge-size: 1.5rem;
    margin-left: 10px;
  }

  .verified-badge .badge-frame {
    box-shadow: 0px 0px 10px 0px rgba(0, 0, 0, 0.2);
  }

  .profile-bio {
    /* 修复换行符不显示的问题 */
    white-space: pre-wrap;
    /* 防止用户输入超长连续字符撑爆布局 */
    word-break: break-word;
    margin-bottom: 20px;
    font-size: 1rem;
    line-height: 1.5;
  }

  .profile-tags {
    margin-bottom: 20px;
  }

  .profile-stats {
    margin: 1rem 0;
  }

  .expand-more {
    display: flex;
    font-size: 0.9rem;
  }

  .expand-more img {
    width: 1.2rem;
    margin-left: 4px;
    transform: rotate(-180deg);
    filter: invert(50%) sepia(100%) saturate(250%) hue-rotate(180deg);
  }
}
</style>
