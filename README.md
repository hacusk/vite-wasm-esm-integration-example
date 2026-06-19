# vite-wasm-esm-integration-example

Vite の [WebAssembly サポート](https://vite.dev/guide/features#webassembly)を使って、Rust 製 WASM を Vue コンポーネントからインポートする 2 つのアプローチのデモ。

## 構成

```
.
├── wasm/                      # Rust — WASM ソース（共通）
├── vite-wasm-instance/        # アプローチ① ?init
└── vite-wasm-esm-integration/ # アプローチ② ESM 統合
```

## 2 つのアプローチ

### `vite-wasm-instance` — `?init`

Vite 組み込みの機能。追加プラグイン不要。`init()` を呼んで `WebAssembly.Instance` を取得し、`exports` から関数を取り出す。

```ts
import wasmInit from '../assets/main.wasm?init'

const instance = await wasmInit()
const add = instance.exports['add'] as (a: number, b: number) => number
```

top-level await を使うため、コンポーネントが非同期になる。親で `<Suspense>` が必要。

### `vite-wasm-esm-integration` — ESM 統合

[WebAssembly/ES Module Integration 提案](https://github.com/WebAssembly/esm-integration)に準拠した形。通常の named import と同じ感覚で書ける。

```ts
import { add, fibonacci } from '../assets/main.wasm'
```

Vite 8 単体ではブロックされるため [vite-plugin-wasm](https://github.com/Menci/vite-plugin-wasm) が必要。

```ts
// vite.config.ts
import wasm from 'vite-plugin-wasm'
export default defineConfig({ plugins: [wasm(), vue()] })
```

TypeScript の named import に型を付けるには `.wasm.d.ts` と `allowArbitraryExtensions` が必要。

```ts
// src/assets/main.wasm.d.ts
export declare function add(a: number, b: number): number
export declare function fibonacci(n: number): number
```

```json
// tsconfig.app.json
{ "compilerOptions": { "allowArbitraryExtensions": true } }
```

## なぜ Rust か

`wasm32-unknown-unknown` ターゲットはランタイムを含まない純粋な WASM を出力する。  
Go/TinyGo は `wasm_exec.js` などのランタイム shim が必要なため、`import { add }` の形では使えない。

| | TinyGo | Rust (`wasm32-unknown-unknown`) |
|---|---|---|
| WASM サイズ | 21 KB | 500 B |
| JS ランタイム | `wasm_exec.js` 必須 | 不要 |
| WASM imports | `go.*` 多数 | ゼロ |

## セットアップ

**必要なもの**

- [Rust](https://rustup.rs/) + `wasm32-unknown-unknown` ターゲット
- [Node.js](https://nodejs.org/) + [pnpm](https://pnpm.io/)

```sh
rustup target add wasm32-unknown-unknown
```

**WASM をビルド**

```sh
cd wasm
cargo wasm
# → target/wasm32-unknown-unknown/release/wasm_demo.wasm
```

各プロジェクトの `src/assets/main.wasm` にコピーして使う。

**開発サーバー起動**

```sh
# ?init 版
cd vite-wasm-instance
pnpm install && pnpm dev

# ESM 統合版
cd vite-wasm-esm-integration
pnpm install && pnpm dev
```

## Rust のコード

```rust
// wasm/src/lib.rs
#[unsafe(no_mangle)]   // edition 2024 では unsafe が必要
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[unsafe(no_mangle)]
pub fn fibonacci(n: i32) -> i32 { /* ... */ }
```

```toml
# wasm/.cargo/config.toml
[alias]
wasm = "build --target wasm32-unknown-unknown --release"
```
