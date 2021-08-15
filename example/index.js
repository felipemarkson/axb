import init, { solve, complex_solve } from '../pkg/axb.js';

async function run() {
    // First up we need to actually load the wasm file, so we use the
    // default export to inform it where the wasm file is located on the
    // server, and then we wait on the returned promise to wait for the
    // wasm to be loaded.
    //
    // It may look like this: `await init('./pkg/without_a_bundler_bg.wasm');`,
    // but there is also a handy default inside `init` function, which uses
    // `import.meta` to locate the wasm file relatively to js file.
    //
    // Note that instead of a string you can also pass in any of the
    // following things:
    //
    // * `WebAssembly.Module`
    //
    // * `ArrayBuffer`
    //
    // * `Response`
    //
    // * `Promise` which returns any of the above, e.g. `fetch("./path/to/wasm")`
    //
    // This gives you complete control over how the module is loaded
    // and compiled.
    //
    // Also note that the promise, when resolved, yields the wasm module's
    // exports which is the same as importing the `*_bg` module in other
    // modes
    await init();
    let A = {
        ncols: 3,
        nrows: 3,
        elements: [
            {
                col: 0,
                row: 0,
                value: 1.
            },
            {
                col: 1,
                row: 1,
                value: -1.
            },
            {
                col: 2,
                row: 2,
                value: -1.
            },

        ]
    }

    let b = {
        ncols: 1,
        nrows: 3,
        elements: [
            {
                col: 0,
                row: 0,
                value: 1.
            },
            {
                col: 0,
                row: 1,
                value: 1.
            },
            {
                col: 0,
                row: 2,
                value: 1.
            },

        ]
    }
    console.log(solve(A, b))

}

run()