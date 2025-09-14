import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Project } from '@/types'

export const useProjectStore = defineStore('project', () => {
  // State
  const currentProject = ref<Project | null>(null)
  const recentProjects = ref<Project[]>([])
  const isDirty = ref(false)
  const isSaving = ref(false)

  // Getters
  const hasProject = computed(() => currentProject.value !== null)
  const projectName = computed(() => currentProject.value?.name || 'Untitled')

  // Actions
  function createProject(name: string) {
    currentProject.value = {
      id: crypto.randomUUID(),
      name,
      path: '',
      createdAt: new Date(),
      modifiedAt: new Date(),
      version: '1.0.0',
      schematic: {
        sheets: [],
        components: [],
        wires: [],
        nets: [],
        buses: []
      },
      pcb: {
        layers: [],
        components: [],
        traces: [],
        vias: [],
        pads: [],
        keepouts: [],
        dimensions: {
          width: 100,
          height: 100,
          thickness: 1.6,
          origin: { x: 0, y: 0 }
        }
      },
      libraries: [],
      settings: {
        units: 'mm',
        gridSize: 1,
        snapToGrid: true,
        autoSave: true,
        autoSaveInterval: 300
      }
    }
    isDirty.value = false
  }

  function openProject(project: Project) {
    currentProject.value = project
    isDirty.value = false

    // Add to recent projects
    const index = recentProjects.value.findIndex(p => p.id === project.id)
    if (index > -1) {
      recentProjects.value.splice(index, 1)
    }
    recentProjects.value.unshift(project)

    // Keep only last 10 recent projects
    if (recentProjects.value.length > 10) {
      recentProjects.value = recentProjects.value.slice(0, 10)
    }
  }

  async function saveProject() {
    if (!currentProject.value) return

    isSaving.value = true
    try {
      // TODO: Implement actual save logic with Tauri
      currentProject.value.modifiedAt = new Date()
      isDirty.value = false
      console.log('Project saved:', currentProject.value)
    } finally {
      isSaving.value = false
    }
  }

  function closeProject() {
    currentProject.value = null
    isDirty.value = false
  }

  function markDirty() {
    isDirty.value = true
  }

  function openSchematic() {
    console.log('Opening schematic editor')
  }

  function openPCB() {
    console.log('Opening PCB editor')
  }

  const selectedComponentId = ref<string | null>(null)

  function updateComponentProperty(
    componentId: string,
    property: string,
    value: string | number | boolean
  ) {
    if (!currentProject.value) return
    console.log(`Updating component ${componentId} property ${property} to ${value}`)
    markDirty()
  }

  return {
    // State
    currentProject,
    recentProjects,
    isDirty,
    isSaving,
    selectedComponentId,

    // Getters
    hasProject,
    projectName,

    // Actions
    createProject,
    openProject,
    saveProject,
    closeProject,
    markDirty,
    openSchematic,
    openPCB,
    updateComponentProperty
  }
})
