// @/config/sidebar/menu.config.ts
import { getIconUrl } from '@fuyeor/commons';
import { useAuth } from '@/composables/auth/useAuth';
import type { SidebarItemConfig } from '@fuyeor/interactify';

export function useMenuConfig() {
  const { currentUser } = useAuth();

  const signedOutNavItemsRaw: SidebarItemConfig[] = [
    {
      target: '/welcome',
      icon: getIconUrl('home'),
      textKey: 'home',
    },
  ];

  const signedInNavItemsRaw: SidebarItemConfig[] = [
    {
      target: '/',
      icon: getIconUrl('home'),
      textKey: 'home',
    },
    {
      target: `/@${currentUser.value?.username}`,
      icon: getIconUrl('person'),
      textKey: 'profile',
    },
    {
      target: '/options',
      icon: getIconUrl('settings'),
      textKey: 'settings',
    },
  ];

  return {
    signedOutNavItemsRaw,
    signedInNavItemsRaw,
  };
}
