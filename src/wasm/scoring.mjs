const wasmUrl = new URL('../../public/wasm/lol_scoring_core.wasm', import.meta.url);
const MAX_INPUT_BYTES = 1024 * 1024;
let wasmInstancePromise;

async function loadWasm() {
    if (!wasmInstancePromise) {
        wasmInstancePromise = (async () => {
            const response = await fetch(wasmUrl);
            if (!response.ok) {
                throw new Error(`Rust scoring module request failed (${response.status})`);
            }
            const bytes = await response.arrayBuffer();
            const { instance } = await WebAssembly.instantiate(bytes, {});
            const wasm = instance.exports;
            for (const name of [
                'memory', 'scoring_alloc', 'scoring_dealloc', 'scoring_recommend',
                'scoring_result_ptr', 'scoring_result_len', 'scoring_error_ptr', 'scoring_error_len'
            ]) {
                if (!(name in wasm)) throw new Error(`Rust scoring module is missing export: ${name}`);
            }
            return wasm;
        })().catch(error => {
            wasmInstancePromise = undefined;
            throw error;
        });
    }
    return wasmInstancePromise;
}

function readBuffer(wasm, pointer, length) {
    return new Uint8Array(wasm.memory.buffer, pointer, length).slice();
}

/** Initialize the Rust/Wasm core once and return full scoring results for quiz answers. */
export async function scoreAnswers(answers) {
    const wasm = await loadWasm();
    const input = new TextEncoder().encode(JSON.stringify(answers));
    if (input.byteLength > MAX_INPUT_BYTES) throw new Error('Questionnaire input is too large.');

    const pointer = wasm.scoring_alloc(input.byteLength);
    if (!pointer && input.byteLength !== 0) throw new Error('Unable to allocate Wasm input buffer.');
    try {
        new Uint8Array(wasm.memory.buffer, pointer, input.byteLength).set(input);
        const status = wasm.scoring_recommend(pointer, input.byteLength);
        if (status !== 0) {
            const error = new TextDecoder().decode(readBuffer(
                wasm, wasm.scoring_error_ptr(), wasm.scoring_error_len()
            ));
            throw new Error(error || `Rust scoring failed (${status}).`);
        }
        const resultBytes = readBuffer(wasm, wasm.scoring_result_ptr(), wasm.scoring_result_len());
        return JSON.parse(new TextDecoder().decode(resultBytes));
    } finally {
        wasm.scoring_dealloc(pointer, input.byteLength);
    }
}
