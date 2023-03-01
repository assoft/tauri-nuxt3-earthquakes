# Afad ve Kandilli Rasathanesi Son Depremler Uygulaması (Win/MacOS)

- Stack
    - Tauri (Rust)
    - Nuxt 3 (Vue/Typescript)
    - Unocss

## Özellikler

- Bildirimler depremlerin şiddetine göre özelleştirilebilir.
- Yenileme sıklığı özelleştirilebilir.

## Uygulama
- [Windows x64](https://github.com/assoft/tauri-nuxt3-earthquakes/releases/download/v0.0.8/Earthquakes.Tracker_0.0.1_x64_en-US.msi)
- [MacOS](https://github.com/assoft/tauri-nuxt3-earthquakes/releases/download/v0.0.8/Earthquakes.Tracker_0.0.1_x64.dmg)
- Linux Build [planlanıyor...]
<div align="center">
    <img src="./screenshot.png" width="400px"</img> 
</div>

# Development

## Setup

Make sure to install the dependencies:

```bash
# yarn
yarn install

# npm
npm install

# pnpm
pnpm install
```

## Development Server

Start the development server on http://localhost:3000

```bash
npm/yarn/pnpm tauri dev
```

## Production

Build the application for production:

```bash
npm/yarn/pnpm run tauri build
```
