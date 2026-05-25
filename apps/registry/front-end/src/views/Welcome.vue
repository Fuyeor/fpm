<!-- @/views/Welcome.vue -->
<template>
  <!-- 语言切换器 -->
  <locale-switcher
    :supported-locales="SUPPORTED_LOCALES"
    @change="handleLocaleChange"
  />

  <div class="intro-layout">
    <!-- Hero Section -->
    <section class="hero">
      <div class="container">
        <div class="hero-content">
          <h1 v-html="t('intro.title')"></h1>
          <p>{{ t('intro.desc') }}</p>
          <a class="cta-button" @click="handleSignin">{{
            t('signin.withFu')
          }}</a>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useRoute, useRouter } from '@fuyeor/vue-router';
import { useLocale } from '@fuyeor/locale';
import { LocaleSwitcher } from '@fuyeor/interactify';
import { useAuth } from '@/composables/auth/useAuth';
import { SUPPORTED_LOCALES } from '@/config/locales';

const route = useRoute();
const router = useRouter();

const { t } = useLocale();
const { isAuthenticated } = useAuth();

// 页面挂载时检查用户是否已登录
onMounted(() => {
  if (isAuthenticated.value) {
    router.push({ name: 'Home' });
  }
});

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
:root {
  --primary: #111111;
  --secondary: #666666;
  --accent: #007aff;
  --neutral: #f5f5f5;
  --border: #e0e0e0;
  --shadow: 0 4px 20px rgba(0, 0, 0, 0.05);
}
:root[data-theme='dark'],
:root[data-theme='black'] {
  --primary: #ffffff;
  --secondary: #d6d6d6;
  --accent: #007aff;
  --neutral: #2c2936;
  --border: #e0e0e0;
}
html:lang(zh-hans),
html:lang(zh-hant),
html:lang(zh-cn),
html:lang(zh-tw),
html:lang(zh-hk),
html:lang(ja) {
  .hero {
    .hero-content {
      h1 {
        font-size: clamp(2.5rem, 6vw, 4rem);
      }
      .cta-button {
        letter-spacing: 0.1em;
      }
    }
  }
}
.locale-switcher {
  padding: 20px 0 0 20px;
}
.intro-layout {
  h2 {
    border: none;
    color: var(--primary);
  }
  .container {
    width: 100%;
    margin: 0 auto;
    padding: 0 2rem;
  }
  /* Hero Section */
  .hero {
    display: flex;
    align-items: center;
    padding: 8rem 0;
    .hero-content {
      max-width: 800px;
      margin: 0 auto;
      h1 {
        font-size: clamp(2.5rem, 6vw, 3rem);
        color: var(--primary);
        font-weight: 300;
        margin-bottom: 2rem;
        line-height: 1.2;
        span {
          font-weight: 400;
        }
      }
      p {
        font-size: clamp(1rem, 2vw, 1.25rem);
        color: var(--secondary);
        margin-bottom: 3rem;
        max-width: 600px;
      }
      .cta-button {
        display: inline-block;
        border: 2px solid var(--primary);
        border-radius: 24px;
        padding: 0.8rem 2.5rem;
        text-decoration: none;
        font-size: 1rem;
        letter-spacing: 0.05em;
        font-weight: 500;
        color: var(--primary);
        cursor: pointer;
        transition: all 0.3s ease;
        &:hover {
          background: var(--primary);
          color: var(--white);
        }
      }
    }
  }
  /* Features Section */
  .features {
    padding: 6rem 0;
    background: var(--neutral);
    .section-header {
      display: flex;
      margin-bottom: 5rem;
      flex-direction: column;
      align-items: center;
      h2 {
        font-size: clamp(1.8rem, 4vw, 2.5rem);
        font-weight: 300;
        margin-bottom: 1.5rem;
      }
      p {
        color: var(--secondary);
      }
    }
    .features-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
      gap: 4rem;
    }
    .feature-card {
      background: var(--surface);
      padding: 1rem 3rem 2rem 3rem;
      border-radius: var(--radius-md);
      cursor: pointer;
      transition:
        transform 0.3s ease,
        box-shadow 0.3s ease;
      &:hover {
        transform: translateY(-10px);
        box-shadow: var(--shadow);
      }
      .feature-icon {
        font-size: 2.5rem;
        color: var(--accent);
        margin-bottom: 2rem;
      }
      h3 {
        font-size: 1.5rem;
        font-weight: 500;
        margin-bottom: 1.5rem;
      }
      p {
        color: var(--secondary);
        margin-bottom: 1.5rem;
      }
      ul {
        list-style-type: none;
        li {
          color: var(--secondary);
          margin-bottom: 0.5rem;
          font-size: 0.95rem;
          &:before {
            content: '•';
            color: var(--accent);
            font-weight: bold;
            display: inline-block;
            width: 1em;
            margin-left: -1em;
          }
        }
      }
    }
  }
  /* About Section */
  .about {
    padding: 10rem 0;
    .about-content {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 6rem;
      align-items: center;
    }
    .about-text {
      h2 {
        font-size: clamp(1.8rem, 4vw, 2.5rem);
        font-weight: 300;
        margin-bottom: 2rem;
      }
      p {
        color: var(--secondary);
        margin-bottom: 2rem;
      }
      .about-features {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 2rem;
        margin-top: 3rem;
        .about-feature {
          h3 {
            font-size: 1.1rem;
            font-weight: 500;
            margin-bottom: 0.5rem;
          }
          p {
            font-size: 0.9rem;
            margin-bottom: 0;
          }
        }
      }
    }
    .about-image {
      position: relative;
      img {
        width: 100%;
        max-width: 500px;
        height: auto;
        border-radius: 4px;
        box-shadow: var(--shadow);
      }
      .stats-card {
        position: absolute;
        bottom: -3rem;
        left: -3rem;
        background: var(--surface-raised);
        padding: 2rem;
        border-radius: var(--radius-md);
        box-shadow: var(--shadow);
        p:first-child {
          color: var(--secondary);
          font-size: 0.9rem;
          margin-bottom: 0.5rem;
        }
        p:last-child {
          font-size: 2.5rem;
          font-weight: 600;
        }
      }
    }
  }
  /* CTA Section */
  .cta {
    padding: 10rem 0;
    background: var(--primary);
    color: white;

    .cta-content {
      text-align: center;
      h2 {
        font-size: clamp(1.8rem, 4vw, 2.5rem);
        font-weight: 300;
        color: #fff;
        margin-bottom: 2rem;
      }
      p {
        color: rgba(255, 255, 255, 0.8);
        margin-bottom: 3rem;
      }
      .cta-button {
        display: inline-block;
        background: var(--surface-raised);
        color: var(--primary);
        padding: 0.8rem 2.5rem;
        text-decoration: none;
        text-transform: uppercase;
        font-size: 0.85rem;
        letter-spacing: 0.15em;
        font-weight: 500;
        border-radius: 10px;
        transition: all 0.3s ease;
        cursor: pointer;
        &:hover {
          background: var(--accent);
          color: var(--white);
        }
      }
    }
  }
  /* Responsive Styles */
  @media (width <= 900px) {
    .about .about-content {
      grid-template-columns: 1fr;
    }
    .about .about-image {
      order: -1;
      justify-self: center;
    }
  }
  @media (width <= 768px) {
    header nav {
      display: none;
    }
    header .mobile-menu-btn {
      display: block;
    }
    .footer .footer-bottom {
      flex-direction: column;
      gap: 1rem;
      text-align: center;
      .footer-links {
        margin-top: 1rem;
      }
    }
    .hero {
      .hero-content {
        .cta-button {
          padding: 0.5rem 1.5rem;
        }
      }
    }
  }
  @media (width <= 600px) {
    .about .about-features {
      grid-template-columns: 1fr;
    }
    .hero .hero-content h1 {
      font-size: clamp(2rem, 6vw, 3rem);
    }
    .features {
      padding: 6rem 0;
    }
    .about {
      padding: 6rem 0;
    }
    .cta {
      padding: 6rem 0;
    }
  }
}
</style>
