import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Point } from '@/types'
import { schematicAPI } from '@/api'
import { useProjectStore } from './projectStore'

export const useSchematicStore = defineStore('schematic', () => {
  const projectStore = useProjectStore()
  
  // State
  const selectedComponentId = ref<string | null>(null)
  const selectedWireId = ref<string | null>(null)
  const hoveredComponentId = ref<string | null>(null)
  const hoveredWireId = ref<string | null>(null)
  const isDrawingWire = ref(false)
  const wirePoints = ref<Point[]>([])
  const snapToGrid = ref(true)
  const gridSize = ref(10)
  const zoom = ref(1)
  const panOffset = ref({ x: 0, y: 0 })

  // Getters
  const currentSchematic = computed(() => projectStore.currentSchematic)
  
  // Use shallowRef for better performance with large arrays
  const components = computed(() => {
    const schematic = currentSchematic.value
    return schematic ? schematic.components : []
  })
  const wires = computed(() => {
    const schematic = currentSchematic.value
    return schematic ? schematic.wires : []
  })
  
  const selectedComponent = computed(() => {
    if (!selectedComponentId.value) return null
    return components.value.find((c: any) => c.id === selectedComponentId.value)
  })
  
  const selectedWire = computed(() => {
    if (!selectedWireId.value) return null
    return wires.value.find((w: any) => w.id === selectedWireId.value)
  })

  // Actions
  async function addComponent(type: string, x: number, y: number) {
    const schematic = currentSchematic.value
    if (!schematic) {
      throw new Error('No schematic selected')
    }

    try {
      const component = await schematicAPI.addComponent(
        projectStore.currentProject!.id,
        type,
        snapToGrid.value ? snapPosition(x) : x,
        snapToGrid.value ? snapPosition(y) : y
      )
      
      schematic.components.push(component)
      projectStore.markDirty()
      
      return component
    } catch (e) {
      console.error('Failed to add component:', e)
      throw e
    }
  }

  async function updateComponent(componentId: string, properties: Record<string, any>) {
    const schematic = currentSchematic.value
    if (!schematic) return

    try {
      await schematicAPI.updateComponent(projectStore.currentProject!.id, componentId, properties)
      
      const component = schematic.components.find((c: any) => c.id === componentId)
      if (component) {
        Object.assign(component, properties)
        projectStore.markDirty()
      }
    } catch (e) {
      console.error('Failed to update component:', e)
      throw e
    }
  }

  async function deleteComponent(componentId: string) {
    const schematic = currentSchematic.value
    if (!schematic) return

    try {
      await schematicAPI.deleteComponent(projectStore.currentProject!.id, componentId)
      
      const index = schematic.components.findIndex((c: any) => c.id === componentId)
      if (index > -1) {
        schematic.components.splice(index, 1)
        if (selectedComponentId.value === componentId) {
          selectedComponentId.value = null
        }
        projectStore.markDirty()
      }
    } catch (e) {
      console.error('Failed to delete component:', e)
      throw e
    }
  }

  async function moveComponent(componentId: string, x: number, y: number) {
    const schematic = currentSchematic.value
    if (!schematic) return

    try {
      const snappedX = snapToGrid.value ? snapPosition(x) : x
      const snappedY = snapToGrid.value ? snapPosition(y) : y
      
      await schematicAPI.moveComponent(projectStore.currentProject!.id, componentId, snappedX, snappedY)
      
      const component = schematic.components.find((c: any) => c.id === componentId)
      if (component) {
        component.x = snappedX
        component.y = snappedY
        projectStore.markDirty()
      }
    } catch (e) {
      console.error('Failed to move component:', e)
      throw e
    }
  }

  async function rotateComponent(componentId: string, rotation: number) {
    const schematic = currentSchematic.value
    if (!schematic) return

    try {
      await schematicAPI.rotateComponent(projectStore.currentProject!.id, componentId, rotation)
      
      const component = schematic.components.find((c: any) => c.id === componentId)
      if (component) {
        component.rotation = rotation
        projectStore.markDirty()
      }
    } catch (e) {
      console.error('Failed to rotate component:', e)
      throw e
    }
  }

  async function addWire(points: Point[], netName?: string) {
    const schematic = currentSchematic.value
    if (!schematic) {
      throw new Error('No schematic selected')
    }

    try {
      const snappedPoints = snapToGrid.value 
        ? points.map(p => ({ x: snapPosition(p.x), y: snapPosition(p.y) }))
        : points
      
      const wire = await schematicAPI.addWire(projectStore.currentProject!.id, snappedPoints, netName)
      
      schematic.wires.push(wire)
      projectStore.markDirty()
      
      return wire
    } catch (e) {
      console.error('Failed to add wire:', e)
      throw e
    }
  }

  async function deleteWire(wireId: string) {
    const schematic = currentSchematic.value
    if (!schematic) return

    try {
      await schematicAPI.deleteWire(projectStore.currentProject!.id, wireId)
      
      const index = schematic.wires.findIndex((w: any) => w.id === wireId)
      if (index > -1) {
        schematic.wires.splice(index, 1)
        if (selectedWireId.value === wireId) {
          selectedWireId.value = null
        }
        projectStore.markDirty()
      }
    } catch (e) {
      console.error('Failed to delete wire:', e)
      throw e
    }
  }

  function startDrawingWire(x: number, y: number) {
    isDrawingWire.value = true
    wirePoints.value = [{ 
      x: snapToGrid.value ? snapPosition(x) : x, 
      y: snapToGrid.value ? snapPosition(y) : y 
    }]
  }

  function addWirePoint(x: number, y: number) {
    if (!isDrawingWire.value) return
    
    wirePoints.value.push({ 
      x: snapToGrid.value ? snapPosition(x) : x, 
      y: snapToGrid.value ? snapPosition(y) : y 
    })
  }

  async function finishDrawingWire() {
    if (!isDrawingWire.value || wirePoints.value.length < 2) {
      cancelDrawingWire()
      return
    }

    try {
      await addWire(wirePoints.value)
    } finally {
      cancelDrawingWire()
    }
  }

  function cancelDrawingWire() {
    isDrawingWire.value = false
    wirePoints.value = []
  }

  function selectComponent(componentId: string | null) {
    selectedComponentId.value = componentId
    selectedWireId.value = null
  }

  function selectWire(wireId: string | null) {
    selectedWireId.value = wireId
    selectedComponentId.value = null
  }

  function clearSelection() {
    selectedComponentId.value = null
    selectedWireId.value = null
  }

  function setHoveredComponent(componentId: string | null) {
    hoveredComponentId.value = componentId
  }

  function setHoveredWire(wireId: string | null) {
    hoveredWireId.value = wireId
  }

  function setSnapToGrid(snap: boolean) {
    snapToGrid.value = snap
  }

  function setGridSize(size: number) {
    gridSize.value = Math.max(5, Math.min(50, size))
  }

  function setZoom(newZoom: number) {
    zoom.value = Math.max(0.1, Math.min(5, newZoom))
  }

  function setPanOffset(x: number, y: number) {
    panOffset.value = { x, y }
  }

  function snapPosition(value: number): number {
    if (!snapToGrid.value) return value
    return Math.round(value / gridSize.value) * gridSize.value
  }

  async function deleteSelected() {
    if (selectedComponentId.value) {
      await deleteComponent(selectedComponentId.value)
    } else if (selectedWireId.value) {
      await deleteWire(selectedWireId.value)
    }
  }

  return {
    // State
    selectedComponentId,
    selectedWireId,
    hoveredComponentId,
    hoveredWireId,
    isDrawingWire,
    wirePoints,
    snapToGrid,
    gridSize,
    zoom,
    panOffset,
    
    // Getters
    currentSchematic,
    components,
    wires,
    selectedComponent,
    selectedWire,
    
    // Actions
    addComponent,
    updateComponent,
    deleteComponent,
    moveComponent,
    rotateComponent,
    addWire,
    deleteWire,
    startDrawingWire,
    addWirePoint,
    finishDrawingWire,
    cancelDrawingWire,
    selectComponent,
    selectWire,
    clearSelection,
    setHoveredComponent,
    setHoveredWire,
    setSnapToGrid,
    setGridSize,
    setZoom,
    setPanOffset,
    deleteSelected
  }
})