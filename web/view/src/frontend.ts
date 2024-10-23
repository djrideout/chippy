import {
    JsApi,
    Keymap,
    press_key,
    release_key,
} from '../wasm/chippy';

// Use to set functions that are imported into WASM to be called from there
function setWasmImports(imports: Record<string, Function>) {
    window.wasm_imports = {
        ...window.wasm_imports,
        ...imports
    };
}

// Custom ROMs
let roms: Record<string, Uint8Array> = {};

// UI elements
const overlay = document.querySelector('#emulator-overlay') as HTMLDivElement;
const tabs = [...document.querySelectorAll<HTMLDivElement>(".emulator-tab")];
const syncModeSelect = document.querySelector('#sync-mode-select') as HTMLSelectElement;
const romSelect = document.querySelector('#rom-select') as HTMLSelectElement;
const romFileInput = document.querySelector('#rom-file-input') as HTMLInputElement;
const romFileButton = document.querySelector('#rom-file-button') as HTMLButtonElement;
const targetSelect = document.querySelector('#target-select') as HTMLSelectElement;
const clockInput = document.querySelector('#clock-input') as HTMLInputElement;
const resetButton = document.querySelector("#reset-button") as HTMLButtonElement;
const keyElements = [...document.querySelectorAll<HTMLDivElement>('.keypad-key')]
    .sort((a, b) => Number.parseInt(`0x${a.innerText}`, 16) - Number.parseInt(`0x${b.innerText}`, 16));
const keys = keyElements.map((el) => el.getAttribute('key'));

const getSyncMode = () => Number.parseInt(syncModeSelect.value);
const getRom = async () => roms[romSelect.value] ?? new Uint8Array(await (await fetch(romSelect.value)).arrayBuffer());
const getTarget = () => Number.parseInt(targetSelect.value);
const getClock = () => Number.parseInt(clockInput.value);

let _api: JsApi;
async function getAPI() {
    if (!_api) {
        _api = new JsApi(getTarget(), getClock(), await getRom(), new Keymap(keys), getSyncMode());
    }
    return _api;
};

export async function setupUI() {
    const api = await getAPI();

    // Update the key elements with a 'pressed' style when keys are pressed on the WASM side
    setWasmImports({
        on_key_pressed: (i: number) => keyElements[i].classList.add('pressed'),
        on_key_released: (i: number) => keyElements[i].classList.remove('pressed')
    });

    const onReset = async () => {
        api.set_sync_mode(getSyncMode());
        api.load_rom(await getRom());
        api.set_target(getTarget());
        api.set_clock(getClock());
        api.reset();
    };

    // Emulator overlay
    overlay.addEventListener('click', async () => {
        await onReset();
        overlay.remove();
        api.start();
    });

    // Click/touch handlers for key buttons
    for (let keyEl of keyElements) {
        let keyCode = keyEl.getAttribute('key');
        const pressEvents = ['mousedown', 'touchstart'];
        const releaseEvents = ['mouseup', 'mouseleave', 'touchend', 'touchcancel'];
        pressEvents.forEach((event) => keyEl.addEventListener(event, () => press_key(keyCode)));
        releaseEvents.forEach((event) => keyEl.addEventListener(event, () => release_key(keyCode)));
    }

    // Keypad/settings tabs
    tabs.forEach((tab) => tab.addEventListener('click', () => {
        tabs.forEach((currentTab) => {
            let content = document.querySelector(`.emulator-content.${currentTab.classList[1]}`) as HTMLDivElement;
            if (currentTab === tab) {
                currentTab.classList.add('selected');
                content.style.display = '';
            } else {
                currentTab.classList.remove('selected');
                content.style.display = 'none';
            }
        });
    }));

    // Click hidden file input when clicking file input button
    romFileButton.addEventListener('click', () => {
        romFileInput.value = '';
        romFileInput.click();
    });

    // After uploading a ROM, add it to the dropdown and change the value
    romFileInput.addEventListener('change', async (e) => {
        let file = (e.target as HTMLInputElement).files?.[0] as File;
        let rom = new Uint8Array(await file.arrayBuffer());
        roms[file.name] = rom;
        let option = document.createElement('OPTION');
        option.setAttribute('value', file.name);
        option.setAttribute('custom', 'custom');
        option.innerText = file.name;
        romSelect.appendChild(option);
        romSelect.value = file.name;
    });

    // Apply recommended clock rate to clock input when changing target value
    targetSelect.addEventListener('change', (e) => {
        let target = e.target as HTMLSelectElement;
        let option = target.querySelector(`option[value="${target.value}"]`) as HTMLOptionElement;
        clockInput.value = option.getAttribute('clock') ?? '2966';
    });

    // Reset button
    resetButton.addEventListener('click', onReset);
}

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
