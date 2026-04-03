# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Install dependencies
npm install

# Full development (frontend + Rust backend with hot reload)
npm run tauri dev

# Frontend only (Vite dev server on port 1420)
npm run dev

# Production build
npm run tauri build

# TypeScript type check + frontend bundle only
npm run build

# Run Rust unit tests
cargo test  # run from src-tauri/
```

There are no lint commands configured.

## Development Approach

Follow **TDD (Test-Driven Development)**: write tests before implementing features or fixes. Define the expected behavior via tests first, then make them pass.

## Testing

### Rust unit tests
Tests live inline in each module under `#[cfg(test)]`:
- [src-tauri/src/config/mod.rs](src-tauri/src/config/mod.rs) — config read/write, serialization, persistence
- [src-tauri/src/project/mod.rs](src-tauri/src/project/mod.rs) — `get_child_dirs`, vendor extraction, `open_single_dir`

Config tests use `Config::new_with_path(path)` with a temp file to avoid touching the real `%APPDATA%` config.

### Playwright (MCP)
Configured via [.mcp.json](.mcp.json) using `playwright-mcp` (installed globally). Use `npm run dev` to start the Vite server at `http://localhost:1420` before running Playwright tests.

Because `invoke()` requires the Tauri runtime, inject a mock before the page loads:
```js
await page.addInitScript(() => {
  window.__TAURI_INTERNALS__ = {
    invoke: (cmd, args) => {
      if (cmd === 'get_projects') return Promise.resolve([...])
      // ...
    },
    transformCallback: () => {},
    convertFileSrc: (src) => src,
  }
})
await page.goto('http://localhost:1420')
```

## Architecture

This is a **Tauri 2 desktop app** — a React/TypeScript frontend communicating with a Rust backend over Tauri's IPC bridge via `invoke()`.

**Purpose:** Open multiple VSCode instances for all subdirectories of a selected project folder. Optionally run `vtex login {vendor}` before opening.

### Frontend → Backend Communication

Frontend calls Rust via `invoke("command_name", { args })`. The available Tauri commands (defined in [src-tauri/src/lib.rs](src-tauri/src/lib.rs)) are:

| Command | Description |
|---|---|
| `get_config` | Returns `Settings { root_folder, default_login }` |
| `save_root_folder` | Updates the root folder path in config |
| `save_default_login` | Toggles the vendor auto-login preference |
| `get_projects` | Lists child directories of root_folder |
| `get_project_dirs` | Lists child directories of a specific project path |
| `open_project` | Opens all subdirs in VSCode; optionally runs vtex login |
| `open_single_dir` | Opens one specific directory in VSCode (no vtex login) |

### Config Persistence

[src-tauri/src/config/mod.rs](src-tauri/src/config/mod.rs) stores a JSON config file at `%APPDATA%\Manage Projects\config.json`:
```json
{ "root_folder": "...", "default_login": false }
```

### Project Opening Logic

[src-tauri/src/project/mod.rs](src-tauri/src/project/mod.rs):
- `open_project` — extracts vendor from last path segment, optionally spawns `vtex login {vendor}`, then opens all child dirs in VSCode
- `open_single_dir` — opens one specific directory in VSCode, no login
- `open_vscode` uses `CREATE_NO_WINDOW` (0x08000000) so no `cmd.exe` window appears
- `vtex login` still spawns with a visible terminal (`/K`) since it requires user interaction

### Frontend Component Tree

```
App.tsx
└── ProjectList (src/components/project_list/)
    ├── Provides ProjectContext (root_folder, projects list)
    ├── ProjectItem — split card: left 70% opens all subdirs, right 30% arrow opens dropdown
    │   └── Dropdown — lists subdirs fetched via get_project_dirs; click opens one via open_single_dir
    └── Config modal (src/components/config/)
        └── SelectFolder — uses Tauri dialog plugin for native folder picker
```

## Known UI Issues (to be fixed)

Found via Playwright inspection:

1. **Modal doesn't close on ESC or overlay click** — `Modal.tsx` has no keyboard listener and the overlay has no `onClick`.
2. **CSS typo** — `config.module.css` defines `.congfigIcon` instead of `.configIcon`; `filter: invert(1)` and `cursor: pointer` are never applied to the config icon.
3. **Dropdown covers cards in subsequent grid rows** — `position: absolute` + `z-index: 100` overlaps cards below.
4. **Dropdown too narrow (100px)** — subproject names get truncated (e.g. `product-sum...`).
5. **Vendor names truncated without ellipsis indicator** — long names clip silently at 70px.
6. **Modal `min-height: 300px`** — forces ~200px of empty space below the two config items.
7. **`backdrop-filter: blur(0px)` in `.modalBody`** — dead CSS rule, leftover from refactor.

## Key Constraints

- **Windows-only in practice**: Backend uses `cmd.exe` explicitly. Icons include Windows targets but macOS icons exist.
- **Tauri capabilities**: Shell and dialog plugin permissions are defined in [src-tauri/capabilities/](src-tauri/capabilities/). Any new shell commands or dialog usage requires capability declarations.
- **TypeScript strict mode**: `noUnusedLocals` and `noUnusedParameters` are errors — keep imports and parameters clean.
- **Vite port is fixed**: Dev server must run on port 1420 (HMR: 1421) to match `tauri.conf.json`.
