import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Project, SheetSize } from '@/types'
import { projectAPI, fileAPI } from '@/api'

export const useProjectStore = defineStore('project', () => {
  // State
  const currentProject = ref<Project | null>(null)
  const currentSchematicId = ref<string | null>(null)
  const projectPath = ref<string | null>(null)
  const recentProjects = ref<string[]>([])
  const isDirty = ref(false)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const currentSchematic = computed(() => {
    if (!currentProject.value) return null
    return currentProject.value.schematic
  })

  const hasProject = computed(() => currentProject.value !== null)
  const projectName = computed(() => currentProject.value?.name ?? 'Untitled')

  // Actions
  async function createProject(name: string) {
    try {
      isLoading.value = true
      error.value = null
      
      const project = await projectAPI.createProject(name)
      currentProject.value = project
      
      // Create initial schematic
      if (project.schematic.sheets.length > 0) {
        currentSchematicId.value = project.schematic.sheets[0].id
      }
      
      isDirty.value = false
      return project
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      isLoading.value = false
    }
  }

  async function openProject(path: string) {
    try {
      isLoading.value = true
      error.value = null
      
      const project = await projectAPI.openProject(path)
      currentProject.value = project
      projectPath.value = path
      
      // Select first schematic
      if (project.schematic.sheets.length > 0) {
        currentSchematicId.value = project.schematic.sheets[0].id
      }
      
      isDirty.value = false
      addToRecentProjects(path)
      
      return project
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      isLoading.value = false
    }
  }

  async function saveProject() {
    if (!currentProject.value) {
      throw new Error('No project to save')
    }

    try {
      isLoading.value = true
      error.value = null

      if (!projectPath.value) {
        // Need to show save dialog
        const path = await fileAPI.showSaveDialog(`${currentProject.value.name}.eda`)
        if (!path) {
          return false // User cancelled
        }
        projectPath.value = path
      }

      await projectAPI.saveProject(currentProject.value, projectPath.value)
      isDirty.value = false
      addToRecentProjects(projectPath.value)
      
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      isLoading.value = false
    }
  }

  async function saveProjectAs() {
    if (!currentProject.value) {
      throw new Error('No project to save')
    }

    try {
      isLoading.value = true
      error.value = null

      const path = await fileAPI.showSaveDialog(`${currentProject.value.name}.eda`)
      if (!path) {
        return false // User cancelled
      }

      await projectAPI.saveProjectAs(currentProject.value, path)
      projectPath.value = path
      isDirty.value = false
      addToRecentProjects(path)
      
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      isLoading.value = false
    }
  }

  async function addSchematic(name: string) {
    if (!currentProject.value) {
      throw new Error('No project open')
    }

    try {
      isLoading.value = true
      error.value = null

      const schematic = await projectAPI.addSchematicToProject(
        currentProject.value.id,
        name
      )
      
      // Add the schematic as a new sheet
      currentProject.value.schematic.sheets.push({
        id: schematic.id,
        name: schematic.name,
        size: 'A4' as SheetSize,
        components: []
      })
      currentSchematicId.value = schematic.id
      isDirty.value = true
      
      return schematic
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      isLoading.value = false
    }
  }

  function selectSchematic(schematicId: string) {
    if (!currentProject.value) return
    
    const schematic = currentProject.value.schematic.sheets.find((s: any) => s.id === schematicId)
    if (schematic) {
      currentSchematicId.value = schematicId
    }
  }

  function markDirty() {
    isDirty.value = true
  }

  async function loadRecentProjects() {
    try {
      recentProjects.value = await projectAPI.getRecentProjects()
    } catch (e) {
      console.error('Failed to load recent projects:', e)
    }
  }

  function addToRecentProjects(path: string) {
    const index = recentProjects.value.indexOf(path)
    if (index > -1) {
      recentProjects.value.splice(index, 1)
    }
    recentProjects.value.unshift(path)
    if (recentProjects.value.length > 10) {
      recentProjects.value = recentProjects.value.slice(0, 10)
    }
  }

  function closeProject() {
    currentProject.value = null
    currentSchematicId.value = null
    projectPath.value = null
    isDirty.value = false
    error.value = null
  }

  // UI actions for compatibility with existing components
  const selectedComponentId = ref<string | null>(null)

  function openSchematic() {
    console.log('Opening schematic editor')
  }

  function openPCB() {
    console.log('Opening PCB editor')
  }

  function updateComponentProperty(
    componentId: string,
    property: string,
    value: string | number | boolean
  ) {
    if (!currentProject.value) return
    console.log(`Updating component ${componentId} property ${property} to ${value}`)
    markDirty()
  }

  // Initialize
  loadRecentProjects()

  return {
    // State
    currentProject,
    currentSchematicId,
    projectPath,
    recentProjects,
    isDirty,
    isLoading,
    error,
    selectedComponentId,
    
    // Getters
    currentSchematic,
    hasProject,
    projectName,
    
    // Actions
    createProject,
    openProject,
    saveProject,
    saveProjectAs,
    addSchematic,
    selectSchematic,
    markDirty,
    loadRecentProjects,
    closeProject,
    openSchematic,
    openPCB,
    updateComponentProperty
  }
})