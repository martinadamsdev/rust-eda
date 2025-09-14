<template>
  <div class="home-view">
    <n-card title="Welcome to Rust EDA" :bordered="false">
      <n-space vertical size="large">
        <n-h2>Recent Projects</n-h2>
        <n-list v-if="projectStore.recentProjects.length > 0">
          <n-list-item
            v-for="projectPath in projectStore.recentProjects"
            :key="projectPath"
            @click="openProject(projectPath)"
          >
            <n-thing :title="getProjectName(projectPath)" :description="`Path: ${projectPath}`" />
          </n-list-item>
        </n-list>
        <n-empty v-else description="No recent projects" />

        <n-space>
          <n-button type="primary" @click="createNewProject">New Project</n-button>
          <n-button @click="openExistingProject">Open Project</n-button>
        </n-space>
      </n-space>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { NCard, NSpace, NH2, NList, NListItem, NThing, NEmpty, NButton } from 'naive-ui'
import { useProjectStore } from '@/stores'
import { useRouter } from 'vue-router'

const projectStore = useProjectStore()
const router = useRouter()

function createNewProject() {
  projectStore.createProject('New Project')
  router.push('/editor/schematic')
}

function openExistingProject() {
  // TODO: Implement file dialog with Tauri
  console.log('Open project dialog')
}

function openProject(projectPath: string) {
  projectStore.openProject(projectPath)
  router.push('/editor/schematic')
}

function getProjectName(projectPath: string): string {
  return projectPath.split('/').pop()?.replace('.eda', '') || 'Unknown Project'
}
</script>

<style scoped>
.home-view {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}
</style>
