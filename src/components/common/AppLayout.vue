<template>
  <n-layout has-sider class="app-layout">
    <!-- Sidebar -->
    <n-layout-sider
      v-if="uiStore.sidebarVisible"
      bordered
      :collapsed="sidebarCollapsed"
      :collapsed-width="64"
      :width="240"
      :native-scrollbar="false"
      @collapse="sidebarCollapsed = true"
      @expand="sidebarCollapsed = false"
    >
      <Sidebar />
    </n-layout-sider>

    <!-- Main Content -->
    <n-layout>
      <!-- Header/Toolbar -->
      <n-layout-header bordered class="toolbar-header">
        <Toolbar />
      </n-layout-header>

      <!-- Canvas Area -->
      <n-layout-content class="main-content">
        <slot />
      </n-layout-content>
    </n-layout>

    <!-- Properties Panel -->
    <n-layout-sider
      v-if="uiStore.propertiesPanelVisible"
      bordered
      side="right"
      :width="280"
      :native-scrollbar="false"
    >
      <PropertiesPanel />
    </n-layout-sider>
  </n-layout>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { NLayout, NLayoutSider, NLayoutHeader, NLayoutContent } from 'naive-ui'
import { useUIStore } from '@/stores'
import Toolbar from '@/components/toolbar/Toolbar.vue'
import Sidebar from '@/components/common/Sidebar.vue'
import PropertiesPanel from '@/components/properties/PropertiesPanel.vue'

const uiStore = useUIStore()
const sidebarCollapsed = ref(false)
</script>

<style scoped>
.app-layout {
  width: 100%;
  height: 100vh;
}

.toolbar-header {
  height: 48px;
  display: flex;
  align-items: center;
  padding: 0 16px;
}

.main-content {
  background-color: var(--n-color);
  position: relative;
  overflow: hidden;
}
</style>
