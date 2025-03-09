# Rus to Latin Transliterator

This is a Rust-based WebAssembly library for transliterating Russian text into Latin characters.

## 📦 Installation

### Install the package via NPM:
```sh
npm install rus_to_latin
```

### Install the package via Yarn:
```sh
yarn add rus_to_latin
```

### Install the package via Bun:
```sh
bun add rus_to_latin
```

## 🚀 Usage

### **Russian to Latin Transliteration**
The `russian_to_latin` function converts Russian text into Latin characters:
```js
import init, { russian_to_latin } from "rus_to_latin";

async function run() {
    await init();
    console.log(russian_to_latin("Привет мир")); // Output: Privet mir
}

run();
```

### **Latin to Russian Transliteration**
The `latin_to_russian` function converts Latin text back into Russian:
```js
import init, { latin_to_russian } from "rus_to_latin";

async function run() {
    await init();
    console.log(latin_to_russian("Privet mir")); // Output: Привет мир
}

run();
```

## 🔗 License
This project is licensed under the MIT License.