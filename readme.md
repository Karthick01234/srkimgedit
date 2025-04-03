# SRKImgEdit

A lightweight image editing library powered by **Rust + WebAssembly**, providing efficient and high-performance image processing for the **Web**. This library is cross-platform and will later include **Android support via JNI**.

🚧 This project is under active development! We are looking for contributors to help build and improve this open-source library. Join us!

# Features 🚀

# Basic Editing

- ✅ Crop

- ✅ Resize & Scale

- ✅ Rotate & Flip

- ✅ Brightness & Contrast Adjustment

- ✅ Custom Filters Allow Users to Manipulate Pixels Based on RGBA Values

- ✅ Undo and Redo System

# Advanced Filters & Effects

- ✨ AI-Powered Image Upscaling

- 🖌️ AI-Based Background Removal

- ✨ Edge Detection & Cartoon Effects

- 🎭 Face Detection and Beautification

- 🖼️ Background Replacement

# Performance & Platform Support

- ⚡ Rust + WebAssembly + JNI for high performance across Web & Mobile

- 🎨 2D & 3D Effects: Powered by common Rust libraries

  - 2D Graphics libraries: bevy, macroquad, image

  - 3D Rendering libraries: wgpu, glium

- 🧠 AI Features: Integrated with Rust-based AI libraries

  - Machine Learning & AI libraries: candle, tch-rs, tract

- 🖥️ Web: Runs in browsers using WebAssembly (WASM)

- 📱 Android & iOS: Compiled to native code using JNI for seamless mobile support

- 🚀 GPU Acceleration: Uses Rust 3D libraries (wgpu, glium) for efficient rendering and processing

---

## Installation 📥

You can install **SRKImgEdit** using npm:

```sh
npm install srkimgedit
```

Or include it via CDN:

```html
<script src="https://cdn.jsdelivr.net/npm/srkimgedit.js"></script>
```

## Usage 🛠️

```js
import init, { Canvas } from "srkimgedit";

await init();
const canvas = new Canvas(null, 500, 500);
```

## Project Structure 📂

```
In the pkg folder:
│── srkimgedit.js
│── srkimgedit.d.ts

This project is built using Rust and compiled to WebAssembly for web-based image editing.
Future updates will include Android support via JNI.

srkimgedit/
│── src/
|   |─ Components
│   |  ├── plugins/           # Each feature is a plugin
│   |  |   ├── filters/       # Image filters & effects plugin
│   |  |   ├── ai/            # AI-based feature plugins
│   |  |   ├── webgl-opengl/  # WebGL or OpenGL feature plugins
│   |  |   ├── basic/         # Basic feature plugins
|   |  |── pluginmanager.rs   # Manages plugins, allowing registration & execution
|   |  |── Canvas.rs          # Inherits from Plugin Manager, binds plugins to canvas
│── tests/                    # Integration tests
│── docs/                     # Documentation
│── cargo.toml                # Project metadata
│── readme.md                 # Project README
```

## Contributing 🤝

1. **Fork the repo** and create a new branch.
2. Implement your feature and add tests.
3. Submit a **pull request** with a description of your changes.

## Testing 🧪

## Web

Since this library is used for image editing and primarily operates in a front-end environment, testing is performed via an API.

The `tests/` folder contains integration tests for each feature. Each newly developed feature will have a corresponding test endpoint.

To run tests, navigate to the `tests/` folder and execute:

```sh
npm start
```

## Android

_Coming Soon_ 🚀

## Roadmap 🏗️

- [x] Start Library Development ✅ _(Completed)_
- [⚡] WebAssembly-Based Features Development _(In Progress)_
- [ ] JNI-Based Android Support _(Not Started)_

## License 📜

This project is licensed under the MIT License.

## Links 🔗

- **GitHub:** [srkimgedit](https://github.com/Karthick01234/srkimgedit)
- **Issues:** [Report Bugs](https://github.com/Karthick01234/srkimgedit/issues)
- **Suggestions:** [Suggest an Improvement](https://github.com/Karthick01234/srkimgedit/issues/new?assignees=&labels=enhancement&template=feature_request.md&title=)
- **Documentation:** _Coming Soon_ 🚀
