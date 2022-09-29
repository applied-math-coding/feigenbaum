<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

const state = reactive({
  darkIters: 1000,
  a: 0,
  b: 4
})

const canvas = ref<HTMLCanvasElement | null>(null);

onMounted(async () => {
  const [x, y] = await invoke(
    'plot_feigenbaum_diag',
    { a: state.a, b: state.b, darkIters: state.darkIters, nR: canvas.value!.width }
  ) as [number[], number[]];

  const ctx = canvas.value!.getContext('2d')!;
  const [y_min, y_max] = [0, 1];
  x.map((r, i) => transfCoords([r, y[i]], [state.a, state.b], [y_min, y_max]))
    .forEach(([r, v]) => ctx.fillRect(r, v, 1, 1));
});

function transfCoords(
  [x, y]: [number, number],
  [x_min, x_max]: [number, number],
  [y_min, y_max]: [number, number]): [number, number] {
  const w = canvas.value!.width;
  const h = canvas.value!.height;
  return [
    (x - x_min) / (x_max - x_min) * w,
    h - y / (y_max - y_min) * h
  ];
}
</script>

<template>
  <canvas ref="canvas" width="700" height="500"></canvas>
</template>