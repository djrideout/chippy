import './style.less';
import init from '../wasm/chippy';
import { setupFrontend, waitForCanvas } from './frontend';

await init();

const { frontend, width, height } = await setupFrontend();

const emuContainer = document.querySelector<HTMLDivElement>('#emulator-container');
if (emuContainer) {
    emuContainer.style.width = `${width * 5}px`;
    emuContainer.style.height = `${height * 5}px`;
    emuContainer.style.display = 'block';
}

const emuOverlay = emuContainer?.querySelector('#emulator-overlay');
emuOverlay?.addEventListener('click', () => {
    emuOverlay.remove();
    frontend.start();
});

const canvas = await waitForCanvas();

canvas.focus();
