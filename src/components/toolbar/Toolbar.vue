<template>
  <div class="toolbar">
    <n-space>
      <!-- File Operations -->
      <n-button-group>
        <n-button @click="newProject">
          <template #icon>
            <n-icon :component="DocumentOutline" />
          </template>
        </n-button>
        <n-button @click="openProject">
          <template #icon>
            <n-icon :component="FolderOpen" />
          </template>
        </n-button>
        <n-button :disabled="!projectStore.isDirty" @click="saveProject">
          <template #icon>
            <n-icon :component="Save" />
          </template>
        </n-button>
      </n-button-group>

      <!-- Edit Operations -->
      <n-divider vertical />
      <n-button-group>
        <n-button @click="undo">
          <template #icon>
            <n-icon :component="ArrowBack" />
          </template>
        </n-button>
        <n-button @click="redo">
          <template #icon>
            <n-icon :component="ArrowForward" />
          </template>
        </n-button>
      </n-button-group>

      <!-- Tools -->
      <n-divider vertical />
      <n-button-group>
        <n-button
          v-for="tool in tools"
          :key="tool.name"
          :type="uiStore.currentTool === tool.name ? 'primary' : 'default'"
          @click="uiStore.setTool(tool.name)"
        >
          <template #icon>
            <n-icon :component="tool.icon" />
          </template>
        </n-button>
      </n-button-group>

      <!-- View Controls -->
      <n-divider vertical />
      <n-button-group>
        <n-button @click="uiStore.zoomIn">
          <template #icon>
            <n-icon :component="Add" />
          </template>
        </n-button>
        <n-button @click="uiStore.resetZoom">{{ Math.round(uiStore.zoom) }}%</n-button>
        <n-button @click="uiStore.zoomOut">
          <template #icon>
            <n-icon :component="Remove" />
          </template>
        </n-button>
      </n-button-group>
    </n-space>
  </div>
</template>

<script setup lang="ts">
import { NSpace, NButton, NButtonGroup, NDivider, NIcon } from 'naive-ui'
import {
  DocumentOutline,
  FolderOpen,
  Save,
  ArrowBack,
  ArrowForward,
  Add,
  Remove,
  Scan,
  GitBranch,
  Cube,
  Text,
  Move,
  Trash
} from '@vicons/ionicons5'
import { useProjectStore, useUIStore } from '@/stores'

const projectStore = useProjectStore()
const uiStore = useUIStore()

const tools = [
  { name: 'select', icon: Scan },
  { name: 'wire', icon: GitBranch },
  { name: 'component', icon: Cube },
  { name: 'text', icon: Text },
  { name: 'move', icon: Move },
  { name: 'delete', icon: Trash }
] as const

function newProject() {
  projectStore.createProject('New Project')
}

function openProject() {
  // TODO: Implement with Tauri
  console.log('Open project')
}

function saveProject() {
  projectStore.saveProject()
}

function undo() {
  // TODO: Implement undo
  console.log('Undo')
}

function redo() {
  // TODO: Implement redo
  console.log('Redo')
}
</script>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  height: 100%;
}
</style>
