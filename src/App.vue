<script setup lang="ts">
import { NConfigProvider, NMessageProvider, NDialogProvider, NNotificationProvider } from 'naive-ui'
import { onMounted } from 'vue'
import ErrorBoundary from '@/components/common/ErrorBoundary.vue'
import { setupGlobalErrorHandlers } from '@/utils/errorHandler'
import { useUIStore } from '@/stores'

const uiStore = useUIStore()

onMounted(() => {
  setupGlobalErrorHandlers()
})
</script>

<template>
  <n-config-provider :theme-overrides="uiStore.themeOverrides">
    <n-notification-provider>
      <n-message-provider>
        <n-dialog-provider>
          <error-boundary>
            <router-view />
          </error-boundary>
        </n-dialog-provider>
      </n-message-provider>
    </n-notification-provider>
  </n-config-provider>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body,
#app {
  width: 100%;
  height: 100%;
}

body {
  font-family:
    -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans',
    'Helvetica Neue', sans-serif;
}
</style>
