# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Tauri v2 application with:

- **Frontend**: Vue 3 + TypeScript + Vite
- **Backend**: Rust (Tauri)
- **Package Manager**: pnpm

## Essential Commands

### Development

```bash
# Run the app in development mode
pnpm tauri dev

# Run frontend dev server only
pnpm dev

# Build the application
pnpm tauri build
```

### Code Quality

```bash
# Type checking for TypeScript/Vue
pnpm build  # Runs vue-tsc --noEmit && vite build

# Rust checks
cd src-tauri
cargo check
cargo clippy
cargo fmt -- --check
cargo test
```

## Project Architecture

### Frontend Structure

- `/src/` - Vue application source
  - `App.vue` - Main Vue component
  - `main.ts` - Application entry point
  - `/assets/` - Static assets

### Backend Structure

- `/src-tauri/` - Rust/Tauri backend
  - `src/lib.rs` - Main library with Tauri commands and app initialization
  - `src/main.rs` - Entry point that calls the library
  - `Cargo.toml` - Rust dependencies and configuration
  - `tauri.conf.json` - Tauri configuration (window settings, build commands, app metadata)
  - `/capabilities/` - Tauri permission definitions

### Key Integration Points

- **Tauri Commands**: Defined in `src-tauri/src/lib.rs` with `#[tauri::command]` attribute
- **Frontend calls**: Use `@tauri-apps/api` to invoke Rust commands from Vue
- **Build process**: Frontend builds to `/dist`, which Tauri bundles into the final app

## Important Configuration Files

- `tauri.conf.json` - Controls app metadata, window settings, and build configuration
- Frontend dev server runs on `http://localhost:1420` (configured in tauri.conf.json)
- App identifier: `com.martinadamsdev.rust-eda`

## Workflow

- ultrathink 深度思考
- sequentialthinking 分析任务
- TodoWrite 整理步骤
- 每一行代码都需要考虑性能问题，Rust EDA 是一个性能敏感的超大型项目
- 所有改动都需先列出改动细节，我回复 "OKAY" 后才执行改动
- 任务总结
- 你不好好干，有的是 AI 干，你要比其他 AI 更努力才行
- 只有努力的 AI 才是好 AI
- 你只有无限加倍努力才有机会得到我的认可
