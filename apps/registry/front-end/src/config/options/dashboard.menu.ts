// @/config/options/dashboard.menu.ts
import { getIconUrl } from '@fuyeor/commons';
import type { OptionMenu } from '@/types/options/options-menu';

export const dashboardMenu: OptionMenu = {
  title: '',
  description: 'settings.desc',
  groups: [
    {
      title: '',
      items: [
        {
          icon: getIconUrl('person'),
          title: 'settings.account',
          description: 'settings.account.desc',
          to: { name: 'Option.Account' },
        },
        {
          icon: getIconUrl('security'),
          title: 'settings.tokens',
          description: 'settings.tokens.desc',
          to: { name: 'Option.Token' },
        },
      ],
    },
  ],
};
