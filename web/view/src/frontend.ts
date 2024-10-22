import {
    JsApi,
    SyncModes,
    Target,
    Keymap,
    press_key,
    release_key,
} from '../wasm/chippy';
import { setWasmImports } from './utils';

const keyElements = [...document.querySelectorAll<HTMLDivElement>('.keypad-key')]
    .sort((a, b) => Number.parseInt(`0x${a.innerText}`, 16) - Number.parseInt(`0x${b.innerText}`, 16));

for (let keyEl of keyElements) {
    let keyCode = keyEl.getAttribute('key');
    const pressEvents = ['mousedown', 'touchstart'];
    const releaseEvents = ['mouseup', 'mouseleave', 'touchend', 'touchcancel'];
    pressEvents.forEach((event) => keyEl.addEventListener(event, () => press_key(keyCode)));
    releaseEvents.forEach((event) => keyEl.addEventListener(event, () => release_key(keyCode)));
}

const keys = keyElements.map((el) => el.getAttribute('key'));

setWasmImports({
    on_key_pressed: (i: number) => keyElements[i].classList.add('pressed'),
    on_key_released: (i: number) => keyElements[i].classList.remove('pressed')
});

export async function initAPI(
    romUrl = 'nyancat.ch8',
    target = Target.XO,
    clock = 30000
) {
    const rom = new Uint8Array(await (await fetch(romUrl)).arrayBuffer());
    return new JsApi(target, clock, rom, new Keymap(keys), SyncModes.AudioCallback);
};

export async function waitForCanvas() {
    let loadedResolve: (value: unknown) => void;
    const loaded = new Promise((res) => {
        loadedResolve = res;
    });
    const observer = new MutationObserver((mutations) => {
        for (const mutation of mutations) {
            if ([...mutation.addedNodes].some((node) => (node as HTMLElement).tagName === 'CANVAS')) {
                loadedResolve(true);
            }
        }
    });
    observer.observe(document, { childList: true, subtree: true });
    await loaded;
    observer.disconnect();
    return document.querySelector('#emulator canvas') as HTMLCanvasElement;
}
