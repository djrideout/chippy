import './style.less';
import init from '../wasm/chippy';
import { setupFrontend, waitForCanvas } from './frontend';

await init();

const { frontend, width, height } = await setupFrontend();

let emuWrapper = document.querySelector<HTMLDivElement>("#emulator-wrapper");
let emuDiv = emuWrapper?.querySelector<HTMLDivElement>('#emulator');
if (emuWrapper) {
    emuWrapper.style.width = `${width * 4}px`;
    //emuDiv.style.height = `${height * 4}px`;
    emuDiv && (emuDiv.style.display = 'block');
}

const emuOverlay = emuWrapper?.querySelector('#emulator-overlay');
emuOverlay?.addEventListener('click', () => {
    emuOverlay.remove();
    if (emuDiv) emuDiv.style.display = 'block';
    frontend.start();
});

const canvas = await waitForCanvas();

canvas.focus();
