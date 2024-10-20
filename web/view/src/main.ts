import './style.less';
import init from '../wasm/chippy';
import { setupFrontend, waitForCanvas } from './frontend';

await init();

const frontend = await setupFrontend();

let emuWrapper = document.querySelector<HTMLDivElement>("#emulator-wrapper");
let emuDiv = emuWrapper?.querySelector<HTMLDivElement>('#emulator');
emuDiv && (emuDiv.style.display = 'block');

const emuOverlay = emuWrapper?.querySelector('#emulator-overlay');
emuOverlay?.addEventListener('click', () => {
    emuOverlay.remove();
    frontend.start();
});

const canvas = await waitForCanvas();

canvas.focus();
