// @fuyeor/router/src/components/router-link.ts
import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { SignalWatcher } from '@lit-labs/signals';
import { router } from '../index';
import type { RouteLocationRaw } from '../types';

@customElement('router-link')
export class RouterLink extends SignalWatcher(LitElement) {
  @property({ type: Object }) to!: RouteLocationRaw;
  @property({ type: Boolean }) replace = false;
  @property({ type: String }) activeClass = '';
  @property({ type: String }) exactActiveClass = '';

  static styles = css`
    :host {
      display: contents;
    }
    a {
      text-decoration: none;
      color: inherit;
    }
  `;

  #onClick(event: MouseEvent) {
    if (
      event.metaKey ||
      event.ctrlKey ||
      event.shiftKey ||
      event.button !== 0
    ) {
      return;
    }
    event.preventDefault();
    this.replace ? router.replace(this.to) : router.push(this.to);
  }

  render() {
    const currentRoute = router.currentRoute.get();

    // Fail fast if route cannot be resolved
    const resolvedRoute = router.resolve(this.to);

    const currentPath = currentRoute.path || '/';
    const targetPath = resolvedRoute.path || '/';

    const exactClass =
      this.exactActiveClass ||
      router.options.linkExactActiveClass ||
      'exact-active';
    const activeClass =
      this.activeClass || router.options.linkActiveClass || 'active';

    const isExact =
      currentPath === targetPath || currentPath === targetPath + '/';
    let finalClass = '';

    if (isExact) {
      finalClass = `${exactClass} ${activeClass}`;
    } else if (
      targetPath !== '/' &&
      currentPath.startsWith(targetPath) &&
      currentPath.charAt(targetPath.length) === '/'
    ) {
      finalClass = activeClass;
    }

    return html`
      <a
        href="${resolvedRoute.href}"
        class="${finalClass}"
        @click=${this.#onClick}
      >
        <slot></slot>
      </a>
    `;
  }
}
