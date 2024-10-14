import './style.less';
import init, { Chip8, SyncModes, Target, Keymap, create_frontend } from '../wasm/chippy';

await init();

let rom = new Uint8Array(await (await fetch('nyancat.ch8')).arrayBuffer());

let chip8 = new Chip8(Target.XO, 30000, rom);

let emuDiv = document.querySelector<HTMLDivElement>("#emulator");
if (emuDiv) {
    emuDiv.style.width = `${chip8.get_width() * 5}px`;
    emuDiv.style.height = `${chip8.get_height() * 5}px`;
    emuDiv.style.display = 'block';
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
