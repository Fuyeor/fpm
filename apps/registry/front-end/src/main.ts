// @/main.ts
// pnpm -F @webroamer/registry-front-end dev
import './App';

window.document.body.innerHTML = '';

const app = window.document.createElement('registry-welcome');

window.document.body.appendChild(app);
