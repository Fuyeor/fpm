<!-- @/views/Home.vue -->
<template>
  <locale-switcher
    :supported-locales="SUPPORTED_LOCALES"
    @change="handleLocaleChange"
  />

  <div class="intro-layout">
    <!-- Hero Section -->
    <section class="hero">
      <h1 v-html="t('intro.title')"></h1>
      <p>{{ t('intro.desc') }}</p>

      <SearchBar />

      <a v-if="!isAuthenticated" class="cta-button" @click="handleSignin">
        {{ t('signin.withFu') }}
      </a>
    </section>

    <!-- introduction Section -->
    <section v-if="isAuthenticated" class="introduction">
      <h2>{{ t('intro.start') }}</h2>

      <div class="introduction-links">
        <router-link :to="{ name: 'Option.Account' }" class="nav-item">
          {{ t('intro.start.account') }} <img :src="getIconUrl('back')" />
        </router-link>

        <router-link
          :to="{
            name: 'Profile',
            params: { username: currentUser!.username, tab: 'organizations' },
          }"
          class="nav-item"
        >
          {{ t('intro.start.organization') }} <img :src="getIconUrl('back')" />
        </router-link>

        <router-link :to="{ name: 'Option.Token' }" class="nav-item">
          {{ t('intro.start.token') }} <img :src="getIconUrl('back')" />
        </router-link>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { useRoute, useRouter } from '@fuyeor/vue-router';
import { useLocale } from '@fuyeor/locale';
import { getIconUrl } from '@fuyeor/commons';
import { LocaleSwitcher, SearchBar } from '@fuyeor/interactify';
import { useAuth } from '@/composables/auth/useAuth';
import { SUPPORTED_LOCALES } from '@/config/locales';

const route = useRoute();
const router = useRouter();

const { t } = useLocale();
const { isAuthenticated, currentUser } = useAuth();

// 处理语言切换带来的路由变更
const handleLocaleChange = (newLocale: string) => {
  // 使用 replace 替换当前的 locale 参数
  router.replace({
    name: route.name as string,
    params: {
      locale: newLocale, // 路由会自动生成 /ja/signin
    },
    query: route.query,
  });
};

// 处理登录跳转
const handleSignin = () => {
  // 从环境变量中读取配置
  const clientId = import.meta.env.VITE_APP_OAUTH_CLIENT_ID;
  const redirectUri = import.meta.env.VITE_APP_REDIRECT_URI;
  const authBaseUrl = import.meta.env.VITE_APP_AUTH_BASE_URL;

  // 生成 state 防 CSRF
  const state = Math.random().toString(36).substring(2);
  window.localStorage.setItem('oauth_state', state); // 将 state 存入 localStorage，用于后续验证

  // 构建完整的 OAuth 授权 URL
  const authUrl = new URL(authBaseUrl);
  authUrl.searchParams.set('response_type', 'code');
  authUrl.searchParams.set('client_id', clientId);
  authUrl.searchParams.set('redirect_uri', redirectUri);
  authUrl.searchParams.set('state', state);

  // 跳转到 www 主站进行 OAuth 授权
  window.location.href = authUrl.toString();
};
</script>

<style>
html:lang(zh-hans),
html:lang(zh-hant),
html:lang(ja) {
  .hero {
    h1 {
      font-size: 3.2rem;
    }
    .cta-button {
      letter-spacing: 0.1em;
    }
  }
}

.locale-switcher {
  padding: 20px 0 0 20px;
}

.intro-layout {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 2rem;
}

/* Hero Section */
.hero {
  display: flex;
  padding: 4rem 0;
  flex-direction: column;

  .search-bar-container input {
    border-radius: 24px;
    padding: 14px;
  }

  h1 {
    font-size: 3rem;
    color: var(--text-primary);
    font-weight: 300;
    line-height: 1.2;
  }

  p {
    font-size: 1.25rem;
    color: var(--text-secondary);
    margin-bottom: 3rem;
    max-width: 600px;
  }

  .cta-button {
    display: inline-block;
    align-self: flex-start;
    background: var(--text-primary);
    color: #fff;
    border-radius: 24px;
    padding: 0.8rem 2.5rem;
    margin-top: 3rem;
    text-decoration: none;
    font-size: 1rem;
    letter-spacing: 0.05em;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;

    &:hover {
      background: var(--text-primary);
    }
  }

  @media (width <= 768px) {
    .hero {
      .hero-content {
        .cta-button {
          padding: 0.5rem 1.5rem;
        }
      }
    }
  }
  @media (width <= 600px) {
    .hero .hero-content h1 {
      font-size: clamp(2rem, 6vw, 3rem);
    }
  }
}

/* introduction Section */
.introduction {
  h2 {
    color: var(--text-primary);
    font-weight: 450;
  }
  .introduction-links {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    a {
      display: flex;
      gap: 8px;
    }

    img {
      width: 1.5rem;
      transform: rotate(-180deg);
    }
  }
}
</style>
