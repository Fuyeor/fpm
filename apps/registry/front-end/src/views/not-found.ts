// src/views/not-found.ts
import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

@customElement('not-found')
export class NotFound extends LitElement {
  render() {
    return html`呜呜`;
  }
}
