// src/views/welcome.ts
import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

@customElement('fpm-welcome')
export class FpmLandingPage extends LitElement {
  render() {
    return html` <a href="/signin" class="active">Signin</a>`;
  }
}
