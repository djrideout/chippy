import { Chip8, SyncModes, Target, Keymap, create_frontend } from '../wasm/chippy';
import { setWasmImports } from './utils';

const keyElements = [...document.querySelectorAll<HTMLDivElement>('.keypad-key')]
    .sort((a, b) => Number.parseInt(`0x${a.innerText}`, 16) - Number.parseInt(`0x${b.innerText}`, 16));

const keys = keyElements.map((el) => el.getAttribute('key'));

setWasmImports({
    on_key_pressed: (i: number) => keyElements[i].classList.add('pressed'),
    on_key_released: (i: number) => keyElements[i].classList.remove('pressed')
});

export async function setupFrontend() {
    const rom = new Uint8Array(await (await fetch('nyancat.ch8')).arrayBuffer());
    const chip8 = new Chip8(Target.XO, 30000, rom);
    const keymap = new Keymap(keys);
    return create_frontend(chip8, keymap, SyncModes.AudioCallback);
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
