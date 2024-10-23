declare global {
    interface Window {
        wasm_imports: Record<string, Function>;
    }
}

import './style.less';
import initWasm from '../wasm/chippy';
import { setupUI, waitForCanvas } from './frontend';

await initWasm();
await setupUI();
(await waitForCanvas()).focus();
