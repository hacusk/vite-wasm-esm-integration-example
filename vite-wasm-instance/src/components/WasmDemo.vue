<script setup lang="ts">
import { ref } from 'vue'
import wasmInit from '../assets/main.wasm?init'

const instance = await wasmInit()
const add = instance.exports['add'] as (a: number, b: number) => number
const fibonacci = instance.exports['fibonacci'] as (n: number) => number

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

  <pre class="code-snippet"><code>// Vite ?init
import init from './assets/main.wasm?init'

const instance = await init()
const add = instance.exports['add'] as (a: number, b: number) => number
console.log(add(1, 2)) // 3</code></pre>
</template>
