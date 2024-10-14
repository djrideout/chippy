import { Chip8, SyncModes, Target, Keymap, create_frontend } from '../wasm/chippy';

export async function setupFrontend() {
    const rom = new Uint8Array(await (await fetch('nyancat.ch8')).arrayBuffer());

    const chip8 = new Chip8(Target.XO, 30000, rom);

    const width = chip8.get_width();
    const height = chip8.get_height();

    const keymap = new Keymap([
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

    return {
        frontend: create_frontend(chip8, keymap, SyncModes.AudioCallback),
        width,
        height
    };
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
