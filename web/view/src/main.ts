declare global {
    interface Window {
        wasm_imports: Record<string, Function>;
    }
}

import './style.less';
import init from '../wasm/chippy';
import { setupFrontend, waitForCanvas } from './frontend';

await init();

const frontend = await setupFrontend();

const emuOverlay = document.querySelector('#emulator-overlay');
emuOverlay?.addEventListener('click', () => {
    emuOverlay.remove();
    frontend.start();
});

const canvas = await waitForCanvas();

canvas.focus();
