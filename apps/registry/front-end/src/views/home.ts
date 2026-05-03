// src/views/home.ts
import { LitElement, html, css } from 'lit';
import { customElement, state } from 'lit/decorators.js';
import { SignalWatcher } from '@lit-labs/signals';
import { currentUser } from '@/signals/auth';
import { startSigninFlow } from '@/services/auth.service';
import { createToken } from '@/api/auth';

@customElement('fpm-home')
export class FpmHome extends SignalWatcher(LitElement) {
  @state() private _generatedToken = ''; // 存储生成的令牌

  static styles = css`
    :host {
      display: block;
      max-width: 600px;
      margin: 4rem auto;
      font-family: system-ui;
    }
    .card {
      padding: 2rem;
      border-radius: 12px;
      background: #fff;
      box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
      text-align: center;
    }
    .token-box {
      margin-top: 1.5rem;
      padding: 1rem;
      background: #f4f4f4;
      border-radius: 8px;
      word-break: break-all;
      font-family: monospace;
      border: 1px dashed #ccc;
    }
    .warning {
      color: #ff3b30;
      font-size: 0.8rem;
      margin-top: 0.5rem;
    }
    button {
      padding: 10px 24px;
      border-radius: 8px;
      border: none;
      background: #007aff;
      color: #fff;
      cursor: pointer;
      font-weight: 600;
    }
  `;

  private async _handleCreateToken() {
    try {
      const name = `My Device (${new Date().toLocaleDateString()})`;
      const { data } = await createToken({ name });
      this._generatedToken = data.token;
    } catch (err) {
      alert('Failed to generate token');
    }
  }

  render() {
    const user = currentUser.get();

    return html`
      <div class="card">
        ${user
          ? html`
              <h2>你好, ${user.nickname}!</h2>
              <p>身份已确认 (@${user.username})</p>

              ${this._generatedToken
                ? html`
                    <div class="token-box">
                      <strong>您的新令牌:</strong><br />
                      ${this._generatedToken}
                    </div>
                    <p class="warning">
                      ⚠️ 请立即复制并保存，此令牌只会显示一次！
                    </p>
                  `
                : html`
                    <button @click=${this._handleCreateToken}>
                      生成 CLI 发布令牌 (FPM Token)
                    </button>
                  `}
            `
          : html`
              <h1>FPM Registry</h1>
              <p>请先登录以管理您的包和令牌</p>
              <button @click=${startSigninFlow}>去登录</button>
            `}
      </div>
    `;
  }
}
