declare global {
    interface Window {
        wasm_imports: Record<string, Function>;
    }
}

import './style.less';
import init from '../wasm/chippy';
import { initAPI, waitForCanvas } from './frontend';

await init();

let api = await initAPI();

const emuOverlay = document.querySelector('#emulator-overlay');
emuOverlay?.addEventListener('click', () => {
    emuOverlay.remove();
    api.start();
});

const canvas = await waitForCanvas();

canvas.focus();
