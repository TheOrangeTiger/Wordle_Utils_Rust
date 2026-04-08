/* tslint:disable */
/* eslint-disable */

export class ListJsOutput {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    get_bannedatlets(): string[];
    get_bannedlets(): string;
    get_incatlets(): string;
    get_mustinclets(): string;
}

export function get_words(): string[];

export function init_panic_hook(): void;

export function lists_from_input_js(guess: string, bannedlets: string, bannedatlets: string[], mustinclets: string, incatlets: string, input: string): ListJsOutput;

export function suggest_guess(wordlist: string[], bannedlets: string, bannedatlets: string[], mustinclets: string, incatlets: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_listjsoutput_free: (a: number, b: number) => void;
    readonly get_words: () => [number, number];
    readonly init_panic_hook: () => void;
    readonly listjsoutput_get_bannedatlets: (a: number) => [number, number];
    readonly listjsoutput_get_bannedlets: (a: number) => [number, number];
    readonly listjsoutput_get_incatlets: (a: number) => [number, number];
    readonly listjsoutput_get_mustinclets: (a: number) => [number, number];
    readonly lists_from_input_js: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => number;
    readonly suggest_guess: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => [number, number];
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_drop_slice: (a: number, b: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
