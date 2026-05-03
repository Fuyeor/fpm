// @/App.ts
import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

@customElement('fpm-root')
export class FpmRoot extends LitElement {
  render() {
    // The router-view will automatically handle layers and dynamic rendering
    return html`<router-view></router-view>`;
  }
}

import { initializeAuth, handleCallback } from '@/services/auth.service';

const init = async () => {
  const params = new URLSearchParams(window.location.search);
  const code = params.get('code');

  if (code) {
    // ✨ 场景 A：从主站跳回来的“瞬间”
    await handleCallback(code);
  } else {
    // ✨ 场景 B：普通打开页面，尝试“撞门”恢复登录态
    await initializeAuth();
  }
};

init();
