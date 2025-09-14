<template>
  <div class="settings-view p-6">
    <n-card title="Settings" :bordered="false">
      <n-form label-placement="left" label-width="120">
        <n-form-item label="Theme">
          <n-switch v-model:value="isDark" @update:value="toggleTheme">
            <template #checked>Dark</template>
            <template #unchecked>Light</template>
          </n-switch>
        </n-form-item>
        <n-form-item label="Grid">
          <n-switch v-model:value="uiStore.gridVisible" />
        </n-form-item>
        <n-form-item label="Snap to Grid">
          <n-switch v-model:value="uiStore.snapToGrid" />
        </n-form-item>
      </n-form>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { NCard, NForm, NFormItem, NSwitch } from 'naive-ui'
import { useUIStore } from '@/stores'

const uiStore = useUIStore()
const isDark = ref(uiStore.theme === 'dark')

function toggleTheme(value: boolean) {
  uiStore.theme = value ? 'dark' : 'light'
}
</script>

<style scoped>
.settings-view {
  width: 100%;
  height: 100%;
}
</style>
