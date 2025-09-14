import { defineStore } from 'pinia'
import { ref, computed, onMounted } from 'vue'
import type { ComponentLibrary, ComponentSymbol, ComponentCategory, ComponentSearchResult } from '@/types/library'
import * as libraryAPI from '@/api/library'
import type { ComponentTemplate } from '@/api/library'

export const useLibraryStore = defineStore('library', () => {
  // State
  const libraries = ref<libraryAPI.ComponentLibrary[]>([])
  const activeLibraryId = ref<string | null>(null)
  const searchQuery = ref('')
  const selectedCategory = ref<libraryAPI.ComponentCategory | null>(null)
  const recentComponents = ref<ComponentTemplate[]>([])
  const favoriteComponents = ref<Set<string>>(new Set())
  const customLibraries = ref<libraryAPI.ComponentLibrary[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const activeLibrary = computed(() => 
    libraries.value.find(lib => lib.id === activeLibraryId.value) || null
  )

  const allComponents = computed(() => {
    const components: ComponentTemplate[] = []
    libraries.value.forEach(lib => {
      Object.values(lib.components).forEach(comp => {
        components.push(comp)
      })
    })
    return components
  })

  const filteredComponents = computed(() => {
    if (!activeLibrary.value) return []
    
    let components = Object.values(activeLibrary.value.components)

    // Filter by category
    if (selectedCategory.value) {
      components = components.filter(c => c.categoryId === selectedCategory.value.id)
    }

    // Filter by search query
    if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase()
      components = components.filter(c => 
        c.name.toLowerCase().includes(query) ||
        c.description?.toLowerCase().includes(query) ||
        c.keywords.some(k => k.toLowerCase().includes(query))
      )
    }

    return components
  })

  const categories = computed(() => {
    if (!activeLibrary.value) return []
    return activeLibrary.value.categories
  })

  const favoriteComponentsList = computed(() => 
    allComponents.value.filter(c => favoriteComponents.value.has(c.id))
  )

  // Actions
  async function loadLibraries() {
    loading.value = true
    error.value = null
    try {
      const libs = await libraryAPI.getAllLibraries()
      libraries.value = libs
      if (libs.length > 0 && !activeLibraryId.value) {
        activeLibraryId.value = libs[0].id
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load libraries'
      console.error('Failed to load libraries:', err)
    } finally {
      loading.value = false
    }
  }

  function loadLibrary(library: libraryAPI.ComponentLibrary) {
    const existingIndex = libraries.value.findIndex(lib => lib.id === library.id)
    if (existingIndex >= 0) {
      libraries.value[existingIndex] = library
    } else {
      libraries.value.push(library)
    }
  }

  function removeLibrary(libraryId: string) {
    libraries.value = libraries.value.filter(lib => lib.id !== libraryId)
    if (activeLibraryId.value === libraryId) {
      activeLibraryId.value = libraries.value[0]?.id || null
    }
  }

  function setActiveLibrary(libraryId: string) {
    if (libraries.value.some(lib => lib.id === libraryId)) {
      activeLibraryId.value = libraryId
    }
  }

  async function searchComponents(query: string): Promise<ComponentTemplate[]> {
    try {
      const results = await libraryAPI.searchComponents(query)
      return results.map(([_libraryId, component]) => component)
    } catch (err) {
      console.error('Failed to search components:', err)
      return []
    }
  }

  function addToRecent(component: ComponentTemplate) {
    // Remove if already exists
    recentComponents.value = recentComponents.value.filter(c => c.id !== component.id)
    // Add to beginning
    recentComponents.value.unshift(component)
    // Keep only last 20
    if (recentComponents.value.length > 20) {
      recentComponents.value = recentComponents.value.slice(0, 20)
    }
  }

  function toggleFavorite(componentId: string) {
    if (favoriteComponents.value.has(componentId)) {
      favoriteComponents.value.delete(componentId)
    } else {
      favoriteComponents.value.add(componentId)
    }
  }

  async function getComponent(libraryId: string, componentId: string): Promise<ComponentTemplate | null> {
    try {
      return await libraryAPI.getComponentTemplate(libraryId, componentId)
    } catch (err) {
      console.error('Failed to get component:', err)
      return null
    }
  }

  function createCustomComponent(component: ComponentTemplate) {
    // Find or create custom library
    let customLib = customLibraries.value.find(lib => lib.id === 'custom')
    if (!customLib) {
      customLib = {
        id: 'custom',
        name: 'Custom Components',
        version: '1.0.0',
        description: 'User-created custom components',
        categories: [],
        components: {}
      }
      customLibraries.value.push(customLib)
      libraries.value.push(customLib)
    }

    // Add component to custom library
    customLib.components[component.id] = component
  }

  async function importLibrary(file: File) {
    try {
      const text = await file.text()
      const library = JSON.parse(text) as libraryAPI.ComponentLibrary
      loadLibrary(library)
      return library
    } catch (error) {
      console.error('Failed to import library:', error)
      throw new Error('Invalid library file format')
    }
  }

  function exportLibrary(libraryId: string): string {
    const library = libraries.value.find(lib => lib.id === libraryId)
    if (!library) {
      throw new Error('Library not found')
    }
    return JSON.stringify(library, null, 2)
  }

  function clearSearch() {
    searchQuery.value = ''
    selectedCategory.value = null
  }

  // Initialize on mount
  async function initialize() {
    await loadLibraries()
  }

  return {
    // State
    libraries,
    activeLibraryId,
    searchQuery,
    selectedCategory,
    recentComponents,
    favoriteComponents,
    customLibraries,
    loading,
    error,

    // Getters
    activeLibrary,
    allComponents,
    filteredComponents,
    categories,
    favoriteComponentsList,

    // Actions
    initialize,
    loadLibraries,
    loadLibrary,
    removeLibrary,
    setActiveLibrary,
    searchComponents,
    addToRecent,
    toggleFavorite,
    getComponent,
    createCustomComponent,
    importLibrary,
    exportLibrary,
    clearSearch
  }
})