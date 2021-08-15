# Axb
🔢 _**A simple linear solver for real and complex numbers in WebAssembly.**_


## 🛠️ Build

### Requiriments

- [Rust](https://www.rust-lang.org/) >= 1.54
- [wasm-pack](https://www.rust-lang.org/) >= 0.10

#### For tests:
- [Node.js](https://nodejs.org/) >= 14.17

### How to build

Clone this repository, and then run the command bellow in the repository folder:

```
wasm-pack build --target web
```

This command will build the binaries and the JavaScript files on ```./pkg/```.

## 🔬 Test

```
wasm-pack test --node
```

## 🚀 Usage

You can find an exemple of usage in [```./example/```](example).

### Defining Matrices and Vectors

The matrices are defined as below:

```js
//| 1.0   0.0 |
//| 0.0   1.0 |
const eye2x2 = {
    nrow: 2,
    ncol: 2,
    elements: [
        {
            row: 0,
            col: 0,
            // If the element is real:
            value: 1.0,
            // If the element is complex (a + jb):
            // value: (a, b),
        },
        {
            row: 1,
            col: 1,
            // If the element is real:
            value: 1.0,
            // If the element is complex (a + jb):
            // value: (a, b),
        }
    ]
}
```

Vectors are defined as a Matrix with ```ncols: 1```.

> _**NOTE:**_ You only need to define non-zero values. 

> _**NOTE:**_ If some of the matrix's element is complex, all elements must be defined as complex.



### Solving linear systems


Lets defining ```A``` a nxn matrix ```b``` a vector nx1. The linear system is defined by ``` Ax = b```.


After build the ``` Axb```, we can solve this linear system as below:

```js
import init, { solve, complex_solve } from 'path/to/axb.js';
async () -> {
    const A = //...
    const b = //...
    await init();
    const x = solve(A, b); // For real elements
    // const x = complex_solve(A, b); // For complex elements
}()
```
The returns of these functions are Matrices as well.

> _**NOTE:**_ ```A``` and ```b``` must have the same type of elements (real or complex).


## 📖 License
Axb is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE_APACHE](LICENSE_APACHE) and [LICENSE_MIT](LICENSE_MIT)for details.