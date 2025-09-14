<template>
  <div class="editor-view">
    <AppLayout>
      <router-view />
    </AppLayout>
  </div>
</template>

<script setup lang="ts">
import AppLayout from '@/components/common/AppLayout.vue'
</script>

<style scoped>
.editor-view {
  width: 100%;
  height: 100vh;
}
</style>
