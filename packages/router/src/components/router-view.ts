// @fuyeor/router/src/components/router-view.ts
import { LitElement, html, css, nothing } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { SignalWatcher } from '@lit-labs/signals';
import { router } from '../index';
import type { RouteLocation } from '../types';

@customElement('router-view')
export class RouterView extends SignalWatcher(LitElement) {
  protected override createRenderRoot() {
    return this;
  }

  @property({ type: Number }) depth = 0;
  @property({ attribute: false }) layerContext?: RouteLocation;

  static styles = css`
    :host {
      display: contents;
    }
  `;

  #renderComponent(route: RouteLocation, level: number) {
    const match = route.matched[level];
    if (!match) return nothing;

    // If no component (nesting/grouping), pass through to next router-view
    if (!match.component) {
      return html`<router-view
        .depth=${level + 1}
        .layerContext=${route}
      ></router-view>`;
    }

    // Create element and attach context
    const element = window.document.createElement(match.component);

    (element as any).__routeContext = route;
    (element as any).__routeDepth = level;

    // Direct property assignment is 100x faster than attr observation
    const props = this.#getProps(route, level);
    window.Object.assign(element, props);

    return element;
  }

  #getProps(route: RouteLocation, level: number) {
    const config = route.matched[level]?.props;
    if (!config) return {};
    if (config === true) return { ...route.params };
    if (typeof config === 'function') return config(route);
    return config;
  }

  render() {
    // Top level handles the LayerStack (Background + Modals)
    if (this.depth === 0 && !this.layerContext) {
      return router.layerStack
        .get()
        .map((layer) => this.#renderComponent(layer, 0));
    }

    // Nested views handle their specific depth
    const activeRoute = this.layerContext || router.currentRoute.get();
    return this.#renderComponent(activeRoute, this.depth);
  }
}
