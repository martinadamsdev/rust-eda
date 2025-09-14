import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { GlobalThemeOverrides } from 'naive-ui'

export type Tool = 'select' | 'wire' | 'component' | 'text' | 'move' | 'delete'
export type View = 'schematic' | 'pcb' | 'simulation' | '3d'

export const useUIStore = defineStore('ui', () => {
  // State
  const currentView = ref<View>('schematic')
  const currentTool = ref<Tool>('select')
  const sidebarVisible = ref(true)
  const propertiesPanelVisible = ref(true)
  const theme = ref<'light' | 'dark'>('light')
  const zoom = ref(100)
  const gridVisible = ref(true)
  const snapToGrid = ref(true)

  // Actions
  function setView(view: View) {
    currentView.value = view
  }

  function setTool(tool: Tool) {
    currentTool.value = tool
  }

  function toggleSidebar() {
    sidebarVisible.value = !sidebarVisible.value
  }

  function togglePropertiesPanel() {
    propertiesPanelVisible.value = !propertiesPanelVisible.value
  }

  function toggleTheme() {
    theme.value = theme.value === 'light' ? 'dark' : 'light'
  }

  function setZoom(value: number) {
    zoom.value = Math.max(10, Math.min(500, value))
  }

  function zoomIn() {
    setZoom(zoom.value * 1.2)
  }

  function zoomOut() {
    setZoom(zoom.value / 1.2)
  }

  function resetZoom() {
    zoom.value = 100
  }

  function toggleGrid() {
    gridVisible.value = !gridVisible.value
  }

  function toggleSnapToGrid() {
    snapToGrid.value = !snapToGrid.value
  }

  const themeOverrides = computed<GlobalThemeOverrides>(() => ({
    common: {
      primaryColor: '#18a058',
      primaryColorHover: '#1fb359',
      primaryColorPressed: '#15944a',
      primaryColorSuppl: '#18a058',
      errorColor: '#d03050',
      errorColorHover: '#de576d',
      errorColorPressed: '#ab1f3f',
      errorColorSuppl: '#d03050',
      warningColor: '#f0a020',
      warningColorHover: '#fcb040',
      warningColorPressed: '#c97c10',
      warningColorSuppl: '#f0a020',
      successColor: '#18a058',
      successColorHover: '#1fb359',
      successColorPressed: '#15944a',
      successColorSuppl: '#18a058',
      infoColor: '#2080f0',
      infoColorHover: '#4098fc',
      infoColorPressed: '#1060c9',
      infoColorSuppl: '#2080f0'
    }
  }))

  return {
    // State
    currentView,
    currentTool,
    sidebarVisible,
    propertiesPanelVisible,
    theme,
    zoom,
    gridVisible,
    snapToGrid,
    themeOverrides,

    // Actions
    setView,
    setTool,
    toggleSidebar,
    togglePropertiesPanel,
    toggleTheme,
    setZoom,
    zoomIn,
    zoomOut,
    resetZoom,
    toggleGrid,
    toggleSnapToGrid
  }
})
