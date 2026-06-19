<script setup lang="ts">
import { ref } from 'vue'
import { add, fibonacci } from '../assets/main.wasm'

const addA = ref(3)
const addB = ref(4)
const fibN = ref(10)
</script>

<template>
  <div class="wasm-demo">
    <div class="demo-card">
      <h2>add(a, b)</h2>
      <div class="inputs">
        <input v-model.number="addA" type="number" />
        <span>+</span>
        <input v-model.number="addB" type="number" />
        <span>=</span>
        <output class="result">{{ add(addA, addB) }}</output>
      </div>
    </div>

    <div class="demo-card">
      <h2>fibonacci(n)</h2>
      <div class="inputs">
        <span>fib(</span>
        <input v-model.number="fibN" type="number" min="0" max="40" />
        <span>)</span>
        <span>=</span>
        <output class="result">{{ fibonacci(Math.min(fibN, 40)) }}</output>
      </div>
    </div>
  </div>

  <pre class="code-snippet"><code>// Rust → main.wasm
#[unsafe(no_mangle)]
pub fn add(a: i32, b: i32) -> i32 { a + b }

// Vite ESM統合
import { add, fibonacci } from './assets/main.wasm'
console.log(add(1, 2)) // 3</code></pre>
</template>
