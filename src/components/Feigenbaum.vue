<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";
import Button from 'primevue/button';
import InputNumber from 'primevue/inputnumber';

const state = reactive({
  darkIters: 49000,
  a: 0,
  b: 4,
  simulating: false,
  hoveredR: undefined as number | undefined
})

const canvas = ref<HTMLCanvasElement | null>(null);

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

async function handleSimulate() {
  state.simulating = true;
  const [x, y] = await invoke(
    'plot_feigenbaum_diag',
    { a: state.a, b: state.b, darkIters: state.darkIters, nR: canvas.value!.width }
  ) as [number[], number[]];
  const ctx = canvas.value!.getContext('2d')!;
  ctx.clearRect(0, 0, canvas.value!.width, canvas.value!.height);
  const [y_min, y_max] = [0, 1];
  x.map((r, i) => transfCoords([r, y[i]], [state.a, state.b], [y_min, y_max]))
    .forEach(([r, v]) => ctx.fillRect(r, v, 1, 1));
  state.simulating = false;
}

function handleMouseMove({ offsetX }: MouseEvent) {
  state.hoveredR = state.a + (state.b - state.a) / canvas.value!.width * offsetX;
}

function handleCanvasClick() {
  if (state.hoveredR != undefined) {
    const s = (state.b - state.a) / 4;
    state.a = Math.max(state.hoveredR - s, 0);
    state.b = Math.min(state.hoveredR + s, 4);
  }
}
</script>

<template>
  <h1 class="text-center">Bifurcation Diagram</h1>
  <div class="control">
    <div>
      Dark Iterations:
      <InputNumber v-model="state.darkIters" :min="500" :max="50000" :allowEmpty="false" :disabled="state.simulating" />
    </div>
    <div>
      Interval:
      [
      <InputNumber v-model="state.a" :min="0" mode="decimal" :maxFractionDigits="10" :max="state.b" :allowEmpty="false"
        :disabled="state.simulating" />,
      <InputNumber v-model="state.b" :min="state.a" :max="4" mode="decimal" :maxFractionDigits="10" :allowEmpty="false"
        :disabled="state.simulating" />]
    </div>
  </div>
  <div class="canvas-positioner">
    <div class="text-xs text-color-secondary">Click into area to center interval around r.</div>
    <canvas ref="canvas" width="700" height="500" @mousemove="!state.simulating && handleMouseMove($event)"
      @mouseleave="state.hoveredR=undefined" @click="!state.simulating && handleCanvasClick()"></canvas>
    <div class="canvas-footer">
      <Button label="Simulate" @click="handleSimulate()" :loading="state.simulating" />
      <div class="hovered-r text-xs" v-if="state.hoveredR != undefined">r = {{state.hoveredR}}</div>
    </div>
  </div>

</template>

<style lang="scss" scoped>
.control {
  display: flex;
  gap: 10px;
}

.canvas-positioner {
  canvas {
    cursor: pointer;
    border: 1px solid #cccccc;
  }

  .canvas-footer {
    display: flex;
    justify-content: space-between;

    .hovered-r {
      color: var(--text-color-secondary);
    }
  }
}
</style>