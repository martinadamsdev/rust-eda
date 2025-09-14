<template>
  <div class="simulation-view">
    <n-card title="Simulation" :bordered="false">
      <n-empty description="Simulation - Coming Soon" />
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { NCard, NEmpty } from 'naive-ui'
</script>

<style scoped>
.simulation-view {
  width: 100%;
  height: 100%;
  padding: 20px;
}
</style>
