<template>
  <div class="loading-view">
    <n-spin size="large">
      <template #description>
        {{ message }}
      </template>
    </n-spin>
  </div>
</template>

<script setup lang="ts">
import { NSpin } from 'naive-ui'

defineProps<{
  message?: string
}>()
</script>

<style scoped>
.loading-view {
  width: 100%;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
