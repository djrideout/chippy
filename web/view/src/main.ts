import './style.less';
import init, { Chip8, SyncModes, Target, Keymap, create_frontend } from '../wasm/chippy';

await init();

let rom = new Uint8Array(await (await fetch('nyancat.ch8')).arrayBuffer());

let chip8 = new Chip8(Target.XO, 30000, rom);

let width = chip8.get_width();
let height = chip8.get_height();

let emuContainer = document.querySelector<HTMLDivElement>('#emulator-container');
if (emuContainer) {
    emuContainer.style.width = `${width * 5}px`;
    emuContainer.style.height = `${height * 5}px`;
    emuContainer.style.display = 'block';
}

let keymap = new Keymap([
    'X',
    'Key1',
    'Key2',
    'Key3',
    'Q',
    'W',
    'E',
    'A',
    'S',
    'D',
    'Z',
    'C',
    'Key4',
    'R',
    'F',
    'V'
]);

let frontend = create_frontend(chip8, keymap, SyncModes.AudioCallback);

let overlay = document.querySelector('#emulator-overlay');
overlay?.addEventListener('click', () => {
    overlay.remove();
    frontend.start();
});

let loadedResolve: (value: unknown) => void;
let loaded = new Promise((res) => {
    loadedResolve = res;
});

let observer = new MutationObserver((mutations) => {
    for (const mutation of mutations) {
        if ([...mutation.addedNodes].some((node) => (node as HTMLElement).tagName === 'CANVAS')) {
            loadedResolve(true);
        }
    }
});
observer.observe(emuContainer as Node, { childList: true, subtree: true });
await loaded;
observer.disconnect();
document.querySelector('canvas')?.focus();
