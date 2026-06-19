# vite-wasm-esm-integration-example

Vite の [WebAssembly ESM 統合](https://vite.dev/guide/features#webassembly)を使って、Rust 製 WASM を通常の ES モジュールとして直接インポートするデモ。

```ts
import { add, fibonacci } from './assets/main.wasm'

console.log(add(1, 2))       // 3
console.log(fibonacci(10))   // 55
```

## 構成

```
.
├── wasm/          # Rust — WASM ソース
└── vite/          # Vue + Vite — フロントエンド
```

## なぜ Rust か

`wasm32-unknown-unknown` ターゲットはランタイムを含まない純粋な WASM を出力する。  
Go/TinyGo は `wasm_exec.js` などのランタイム shim が必要で、`import { add }` の形では使えない。

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

**WASM をビルドして Vite にコピー**

```sh
cd wasm
cargo wasm
cp target/wasm32-unknown-unknown/release/wasm_demo.wasm ../vite/src/assets/main.wasm
```

**開発サーバー起動**

```sh
cd vite
pnpm install
pnpm dev
```

## Vite の設定

Vite 8 単体では直接 `.wasm` インポートがブロックされるため、[vite-plugin-wasm](https://github.com/Menci/vite-plugin-wasm) が必要。

```ts
// vite/vite.config.ts
import wasm from 'vite-plugin-wasm'

export default defineConfig({
  plugins: [wasm(), vue()],
})
```

## TypeScript 型

`.wasm.d.ts` ファイルで named import に型を付ける（TypeScript 5.0+ の `allowArbitraryExtensions`）。

```ts
// vite/src/assets/main.wasm.d.ts
export declare function add(a: number, b: number): number
export declare function fibonacci(n: number): number
```

```json
// vite/tsconfig.app.json
{ "compilerOptions": { "allowArbitraryExtensions": true } }
```

型は `wasm-dis`（binaryen）で WASM のエクスポートシグネチャを確認して手書きする。

```sh
wasm-dis vite/src/assets/main.wasm
# → (export "add" (func ...))  type: (param i32 i32) (result i32)
```

## Rust のコード

```rust
// wasm/src/lib.rs
#[unsafe(no_mangle)]   // edition 2024 では unsafe が必要
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub fn fibonacci(n: i32) -> i32 { /* ... */ }
```

`cargo wasm` は `.cargo/config.toml` のエイリアス。

```toml
# wasm/.cargo/config.toml
[alias]
wasm = "build --target wasm32-unknown-unknown --release"
```
