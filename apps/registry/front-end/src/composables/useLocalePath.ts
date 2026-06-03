// @/composables/useLocalePath.ts
import { useRoute } from '@fuyeor/vue-router';

// 根据当前页面的语言前缀，给目标路径加上相同的前缀
export function useLocalePath() {
  const route = useRoute();

  /**
   * @param path 原始路径 (例如 "/photography")
   * @returns 转换后的路径 (例如 "/en/photography")
   */
  const localePath = (path: string): string => {
    // 获取当前的 locale 参数 (en, zh-hans...)
    const currentLocale = route.params.locale as string | undefined;

    // 如果没有 locale，或者目标路径已经是完整的 http 链接，直接返回
    if (!currentLocale || path.startsWith('http')) {
      return path;
    }

    // 确保 path 以 / 开头，方便拼接
    // const cleanPath = path.startsWith('/') ? path : `/${path}`;

    // 拼接前缀
    return `/${currentLocale}${path}`;
  };

  return localePath;
}
