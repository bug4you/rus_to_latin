# Rus to Latin Transliterator

This is a Rust-based WebAssembly library for transliterating Russian text into Latin characters.

## 📦 Installation

Using `wasm-pack`:

```sh
wasm-pack build
```

## 🚀 Usage

`russian_to_latin` function takes a string of Russian text and returns a string of Latin characters.
```js
import init, { russian_to_latin } from "./pkg/rus_to_latin.js";

async function run() {
    await init();
    console.log(russian_to_latin("Привет мир")); // Output: Privet mir
}

run();
```

`latin_to_russian` function takes a string of Latin characters and returns a string of Russian text.
```js
import init, { latin_to_russian } from "./pkg/rus_to_latin.js";

async function run() {
    await init();
    console.log(latin_to_russian("Privet mir")); // Output: Привет мир
}

run();
```

## 🔗 License
This project is licensed under the MIT License.