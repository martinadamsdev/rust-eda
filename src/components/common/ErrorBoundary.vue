<template>
  <div v-if="hasError" class="error-boundary">
    <n-result status="error" title="Something went wrong" :description="errorMessage">
      <template #footer>
        <n-space>
          <n-button type="primary" @click="handleReload">Reload Application</n-button>
          <n-button @click="handleReset">Reset State</n-button>
        </n-space>
      </template>
    </n-result>
  </div>
  <slot v-else />
</template>

<script setup lang="ts">
import { ref, onErrorCaptured } from 'vue'
import { NResult, NButton, NSpace } from 'naive-ui'
import { useRouter } from 'vue-router'

const hasError = ref(false)
const errorMessage = ref('')
const router = useRouter()

onErrorCaptured((error: Error) => {
  console.error('Error captured:', error)
  hasError.value = true
  errorMessage.value = error.message || 'An unexpected error occurred'

  return false
})

function handleReload() {
  window.location.reload()
}

function handleReset() {
  hasError.value = false
  errorMessage.value = ''
  router.push('/')
}
</script>

<style scoped>
.error-boundary {
  width: 100%;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--n-color);
}
</style>
