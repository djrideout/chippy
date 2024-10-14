import './style.less';
import typescriptLogo from './typescript.svg';
import viteLogo from '/vite.svg';
import init, { Chip8, Frontend, SyncModes, Target, Keymap } from '../wasm/chippy';

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <a href="https://vitejs.dev" target="_blank">
      <img src="${viteLogo}" class="logo" alt="Vite logo" />
    </a>
    <a href="https://www.typescriptlang.org/" target="_blank">
      <img src="${typescriptLogo}" class="logo vanilla" alt="TypeScript logo" />
    </a>
    <h1>Vite + TypeScript</h1>
    <p class="read-the-docs">
      Click on the Vite and TypeScript logos to learn more
    </p>
  </div>
`;

window.addEventListener('load', () => {
  init();
  console.log(Chip8);
  console.log(Frontend);
  console.log(SyncModes);
  console.log(Target);
  console.log(Keymap);
});
