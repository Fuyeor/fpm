// @/App.ts
import { LitElement, html, css } from 'lit';
import { customElement } from 'lit/decorators.js';

@customElement('registry-welcome')
export class RegistryWelcome extends LitElement {
  static styles = css`
    :host {
      display: block;
      width: 100dvw;
      height: 100dvh;
    }
  `;

  render() {
    // The router-view will automatically handle layers and dynamic rendering
    return html`<router-view></router-view>`;
  }
}
