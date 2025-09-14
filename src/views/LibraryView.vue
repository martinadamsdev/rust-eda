<template>
  <div class="library-view p-6">
    <n-card title="Component Library" :bordered="false">
      <n-empty description="Component Library - Coming Soon" />
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { NCard, NEmpty } from 'naive-ui'
</script>

<style scoped>
.library-view {
  width: 100%;
  height: 100%;
}
</style>
