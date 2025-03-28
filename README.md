# SRKImgEdit

SRKImgEdit is a lightweight, cross-platform image editing library for Web (JavaScript) and Android (Kotlin). It supports basic to advanced image editing, including real-time filters, AI-powered effects, and GPU-accelerated processing using WebGL and OpenGL ES.

🚧 This project is under active development! We are looking for contributors to help build and improve this open-source library. Join us!

## Features 🚀

### **Basic Editing**

- ✅ Crop
- ✅ Resize & Scale
- ✅ Rotate & Flip
- ✅ Brightness & Contrast Adjustment
- ✅ Multiple Filters by Manipulating Pixel RGBA Values
- ✅ Custom Filters Allow Users to Manipulate Pixels Based on RGBA Values
- ✅ Undo and Redo System

### **Advanced Filters & Effects**

### **AI Features**

- ✨ AI-Powered Image Upscaling
- 🖌️ AI-Based Background Removal
- ✨ Edge Detection & Cartoon Effects
- 🎭 Face Detection and Beautification
- 🖼️ Background Replacement

### **Canvas & Layer System**

- 🖼️ Multiple Layer Support
- ✍️ Drawing, Stickers, and Text Overlay
- 📌 Masking & Transparency

### **WebGL Features**

- ⚡ High-Performance Filters Using WebGL
- 🎭 Image Morphing & Warping
- 🚀 Fast Real-Time Processing

### **Additional Features**

- 📸 Save in Multiple Formats (PNG, JPEG, WebP)
- ✍️ Allow Users to Write Their Own Plugins

## 📌 Repository Structure

This repository serves as the **central hub** for both submodules:

- **[srkimgedit-js](https://github.com/Karthick01234/srkimgedit.js)** → JavaScript library for Web
- **[srkimgedit-android](https://github.com/Karthick01234/srkimgedit.android)** → Kotlin library for Android

When cloning, make sure to initialize submodules:

```

git clone --recurse-submodules https://github.com/Karthick01234/srkimgedit.git

```

If you already cloned the main repo without submodules, run:

```

git submodule update --init --recursive

```

## 🚀 Getting Started

- Clone and Work on a Specific Submodule

For Web (JavaScript)

```

git clone https://github.com/Karthick01234/srkimgedit.js.git

```

For Android (Kotlin)

```

git clone https://github.com/Karthick01234/srkimgedit.android.git

```

## 🔄 Updating the Submodules

Since the main repo is restricted, you will push changes only to the submodule repositories.

Pull the Latest Updates in main repo

```

git pull --recurse-submodules
git submodule update --remote

```

Push Changes to Submodules (You Cannot Push to Main)

```

cd srkimgedit-js # or cd srkimgedit-android
git add .
git commit -m "Your commit message"
git push origin main

```

## 🤝 Contributing

The main repository is read-only for contributors.

To contribute:

- Fork the submodule repo (srkimgedit-js or srkimgedit-android).

- Create a feature branch, make changes, and commit.

- Push the branch and submit a pull request.

## 📜 License

- This project is licensed under the MIT License. See the LICENSE file for more details.

## 📢 Stay Updated

- **🌍 Main Repository:** SRKImgEdit

- **🖥️ Web (JS):** srkimgedit-js

- **📱 Android (Kotlin):** srkimgedit-android
