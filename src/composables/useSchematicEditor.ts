import { ref, computed, onUnmounted } from 'vue'
import type { Ref } from 'vue'
import { useCanvas } from './useCanvas'
import type { ComponentSymbol } from '@/types/library'
import { generateId, snapToGrid } from '@/utils/helpers'
import { useLibraryStore } from '@/stores/library'
import { 
  DIRTY_THRESHOLD,
  SELECTION_PADDING
} from '@/constants/editor'
// Validation imports removed - not currently used

export interface SchematicElement {
  id: string
  type: 'component' | 'wire' | 'junction' | 'label'
  selected: boolean
}

export interface SchematicComponent extends SchematicElement {
  type: 'component'
  x: number
  y: number
  width: number
  height: number
  rotation: number
  reference: string
  value: string
  pins: Array<{
    id: string
    x: number
    y: number
    connected: boolean
  }>
  symbolId?: string
  libraryId?: string
  symbol?: ComponentSymbol
}

export interface SchematicWire extends SchematicElement {
  type: 'wire'
  points: Array<{ x: number; y: number }>
  netName?: string
}

export function useSchematicEditor(canvasRef: Ref<HTMLCanvasElement | null>) {
  const canvas = useCanvas(canvasRef, {
    width: 1600,
    height: 1200,
    backgroundColor: '#f8f8f8',
    gridSize: 10,
    gridColor: '#d0d0d0',
    showGrid: true
  })

  const libraryStore = useLibraryStore()

  const components = ref<SchematicComponent[]>([])
  const wires = ref<SchematicWire[]>([])
  const selectedElements = ref<Set<string>>(new Set())
  const currentTool = ref<'select' | 'wire' | 'component' | 'move' | 'delete'>('select')
  const isDrawingWire = ref(false)
  const currentWire = ref<SchematicWire | null>(null)

  const selectedComponent = computed(() => {
    if (selectedElements.value.size !== 1) return null
    const id = Array.from(selectedElements.value)[0]
    return components.value.find(c => c.id === id) || null
  })
  
  const selectedWire = computed(() => {
    if (selectedElements.value.size !== 1) return null
    const id = Array.from(selectedElements.value)[0]
    return wires.value.find(w => w.id === id) || null
  })
  
  const selectedAnnotation = ref(null)

  function drawComponent(component: SchematicComponent) {
    const ctx = canvas.ctx.value
    if (!ctx) return

    ctx.save()
    ctx.translate(component.x, component.y)
    ctx.rotate((component.rotation * Math.PI) / 180)

    // If component has a symbol from library, draw it
    if (component.symbol) {
      drawComponentSymbol(ctx, component.symbol, component.selected)
    } else {
      // Draw default component body
      ctx.strokeStyle = component.selected ? '#2080f0' : '#000000'
      ctx.lineWidth = component.selected ? 2 : 1
      ctx.fillStyle = '#ffffff'
      ctx.fillRect(-component.width / 2, -component.height / 2, component.width, component.height)
      ctx.strokeRect(-component.width / 2, -component.height / 2, component.width, component.height)

      // Draw pins
      ctx.fillStyle = '#cc0000'
      for (const pin of component.pins) {
        ctx.beginPath()
        ctx.arc(pin.x, pin.y, 3, 0, Math.PI * 2)
        ctx.fill()
      }
    }

    // Draw reference and value
    ctx.fillStyle = '#000000'
    ctx.font = '12px monospace'
    ctx.textAlign = 'center'
    ctx.textBaseline = 'middle'
    ctx.fillText(component.reference, 0, -component.height / 2 - 10)
    ctx.fillText(component.value, 0, component.height / 2 + 10)

    ctx.restore()
  }

  function drawComponentSymbol(ctx: CanvasRenderingContext2D, symbol: ComponentSymbol, selected: boolean) {
    // Draw symbol graphics
    symbol.graphics.symbol.forEach(element => {
      // Set defaults
      ctx.strokeStyle = selected ? '#2080f0' : '#000000'
      ctx.fillStyle = 'none'
      ctx.lineWidth = selected ? 1.5 : 1
      
      // Apply element-specific styles based on type
      if ('stroke' in element && element.stroke) {
        ctx.strokeStyle = element.stroke
      }
      if ('fill' in element && element.fill) {
        ctx.fillStyle = element.fill
      }
      if ('strokeWidth' in element && element.strokeWidth) {
        ctx.lineWidth = element.strokeWidth * (selected ? 1.5 : 1)
      }

      switch (element.type) {
        case 'line':
          ctx.beginPath()
          ctx.moveTo(element.x1, element.y1)
          ctx.lineTo(element.x2, element.y2)
          ctx.stroke()
          break

        case 'rect':
          if (element.fill && element.fill !== 'none') {
            ctx.fillRect(element.x, element.y, element.width, element.height)
          }
          ctx.strokeRect(element.x, element.y, element.width, element.height)
          break

        case 'circle':
          ctx.beginPath()
          ctx.arc(element.cx, element.cy, element.r, 0, Math.PI * 2)
          if (element.fill && element.fill !== 'none') {
            ctx.fill()
          }
          ctx.stroke()
          break

        case 'arc':
          ctx.beginPath()
          const startRad = (element.startAngle * Math.PI) / 180
          const endRad = (element.endAngle * Math.PI) / 180
          ctx.arc(element.cx, element.cy, element.r, startRad, endRad)
          ctx.stroke()
          break

        case 'polygon':
          ctx.beginPath()
          element.points.forEach((point, index) => {
            if (index === 0) {
              ctx.moveTo(point.x, point.y)
            } else {
              ctx.lineTo(point.x, point.y)
            }
          })
          ctx.closePath()
          if (element.fill && element.fill !== 'none') {
            ctx.fill()
          }
          ctx.stroke()
          break

        case 'text':
          ctx.font = `${element.fontSize || 12}px sans-serif`
          ctx.fillStyle = element.fill || '#000000'
          ctx.textAlign = 'center'
          ctx.textBaseline = 'middle'
          ctx.fillText(element.text, element.x, element.y)
          break

        case 'path':
          const path = new Path2D(element.d)
          if (element.fill && element.fill !== 'none') {
            ctx.fill(path)
          }
          ctx.stroke(path)
          break
      }
    })

    // Draw pins
    ctx.fillStyle = selected ? '#2080f0' : '#cc0000'
    symbol.pins.forEach(pin => {
      ctx.beginPath()
      ctx.arc(pin.x, pin.y, 3, 0, Math.PI * 2)
      ctx.fill()
    })
  }

  function drawWire(wire: SchematicWire) {
    const ctx = canvas.ctx.value
    if (!ctx || wire.points.length < 2) return

    ctx.strokeStyle = wire.selected ? '#2080f0' : '#0066cc'
    ctx.lineWidth = wire.selected ? 3 : 2
    ctx.lineCap = 'round'
    ctx.lineJoin = 'round'

    ctx.beginPath()
    ctx.moveTo(wire.points[0].x, wire.points[0].y)
    for (let i = 1; i < wire.points.length; i++) {
      ctx.lineTo(wire.points[i].x, wire.points[i].y)
    }
    ctx.stroke()

    // Draw junction points
    ctx.fillStyle = '#0066cc'
    for (const point of wire.points) {
      ctx.beginPath()
      ctx.arc(point.x, point.y, 2, 0, Math.PI * 2)
      ctx.fill()
    }
  }

  // Differential rendering support
  const dirtyElements = new Set<string>()
  let rafId: number | null = null
  let needsFullRedraw = true

  // markDirty function removed - not currently used
  
  function scheduleRedraw() {
    if (rafId !== null) return
    rafId = requestAnimationFrame(() => {
      performRedraw()
      rafId = null
    })
  }

  function performRedraw() {
    const ctx = canvas.ctx.value
    if (!ctx) return

    if (needsFullRedraw || dirtyElements.size > DIRTY_THRESHOLD) {
      // Full redraw
      canvas.clear()
      
      // Draw all wires
      for (const wire of wires.value) {
        drawWire(wire)
      }
      
      // Draw all components
      for (const component of components.value) {
        drawComponent(component)
      }
      
      needsFullRedraw = false
      dirtyElements.clear()
    } else if (dirtyElements.size > 0) {
      // Differential rendering - only redraw dirty elements
      dirtyElements.forEach(id => {
        const component = components.value.find(c => c.id === id)
        if (component) {
          // Clear the component area with padding
          ctx.clearRect(
            component.x - SELECTION_PADDING,
            component.y - SELECTION_PADDING,
            component.width + SELECTION_PADDING * 2,
            component.height + SELECTION_PADDING * 2
          )
          // Redraw the component
          drawComponent(component)
        }
        
        const wire = wires.value.find(w => w.id === id)
        if (wire) {
          drawWire(wire)
        }
      })
      dirtyElements.clear()
    }
    
    // Always draw current wire on top
    if (currentWire.value) {
      drawWire(currentWire.value)
    }
  }

  function redraw() {
    needsFullRedraw = true
    scheduleRedraw()
  }

  function addComponent(type: string, x: number, y: number) {
    const component: SchematicComponent = {
      id: generateId(),
      type: 'component',
      selected: false,
      x: snapToGrid(x, 10),
      y: snapToGrid(y, 10),
      width: 80,
      height: 60,
      rotation: 0,
      reference: `U${components.value.length + 1}`,
      value: type,
      pins: [
        { id: '1', x: -40, y: -20, connected: false },
        { id: '2', x: -40, y: 0, connected: false },
        { id: '3', x: -40, y: 20, connected: false },
        { id: '4', x: 40, y: -20, connected: false },
        { id: '5', x: 40, y: 0, connected: false },
        { id: '6', x: 40, y: 20, connected: false }
      ]
    }

    components.value.push(component)
    redraw()
    return component
  }

  async function addComponentFromLibrary(symbolId: string, libraryId: string, x: number, y: number) {
    const symbol = await libraryStore.getComponent(symbolId, libraryId)
    if (!symbol) {
      console.error(`Component ${symbolId} not found in library`)
      return null
    }

    // Determine reference prefix based on component type
    const prefixMap: Record<string, string> = {
      'Resistor': 'R',
      'Capacitor': 'C',
      'Inductor': 'L',
      'Diode': 'D',
      'LED': 'D',
      'Transistor': 'Q',
      'MOSFET': 'Q',
      'Op-Amp': 'U',
      'IC': 'U'
    }

    const prefix = prefixMap[symbol.name] || 'U'
    const existingRefs = components.value
      .filter(c => c.reference.startsWith(prefix))
      .map(c => parseInt(c.reference.substring(prefix.length)) || 0)
    const nextNumber = existingRefs.length > 0 ? Math.max(...existingRefs) + 1 : 1

    const component: SchematicComponent = {
      id: generateId(),
      type: 'component',
      selected: false,
      x: snapToGrid(x, 10),
      y: snapToGrid(y, 10),
      width: symbol.symbol?.graphics?.bounds?.width || 100,
      height: symbol.symbol?.graphics?.bounds?.height || 60,
      rotation: 0,
      reference: `${prefix}${nextNumber}`,
      value: String(symbol.defaultProperties?.value || symbol.name),
      pins: (symbol.pins || []).map((pin: any) => ({
        id: pin.id,
        x: pin.x,
        y: pin.y,
        connected: false
      })),
      symbolId,
      libraryId,
      symbol: symbol.symbol
    }

    components.value.push(component)
    redraw()
    return component
  }

  function startWire(x: number, y: number) {
    const snappedX = snapToGrid(x, 10)
    const snappedY = snapToGrid(y, 10)

    currentWire.value = {
      id: generateId(),
      type: 'wire',
      selected: false,
      points: [{ x: snappedX, y: snappedY }]
    }

    isDrawingWire.value = true
  }

  function addWirePoint(x: number, y: number) {
    if (!currentWire.value) return

    const snappedX = snapToGrid(x, 10)
    const snappedY = snapToGrid(y, 10)

    currentWire.value.points.push({ x: snappedX, y: snappedY })
    redraw()
  }

  function finishWire() {
    if (currentWire.value && currentWire.value.points.length >= 2) {
      wires.value.push(currentWire.value)
    }

    currentWire.value = null
    isDrawingWire.value = false
    redraw()
  }
  
  function addWire(wire: SchematicWire) {
    wires.value.push(wire)
    redraw()
    return wire
  }

  function selectElement(id: string, multi: boolean = false) {
    if (!multi) {
      selectedElements.value.clear()
    }

    selectedElements.value.add(id)

    // Update selected state
    components.value.forEach(c => {
      c.selected = selectedElements.value.has(c.id)
    })
    wires.value.forEach(w => {
      w.selected = selectedElements.value.has(w.id)
    })

    redraw()
  }

  function clearSelection() {
    selectedElements.value.clear()
    components.value.forEach(c => (c.selected = false))
    wires.value.forEach(w => (w.selected = false))
    redraw()
  }

  function deleteSelected() {
    components.value = components.value.filter(c => !selectedElements.value.has(c.id))
    wires.value = wires.value.filter(w => !selectedElements.value.has(w.id))
    selectedElements.value.clear()
    redraw()
  }

  function findElementAt(x: number, y: number): SchematicElement | null {
    // Check components first
    for (const component of components.value) {
      const halfWidth = component.width / 2
      const halfHeight = component.height / 2

      if (
        x >= component.x - halfWidth &&
        x <= component.x + halfWidth &&
        y >= component.y - halfHeight &&
        y <= component.y + halfHeight
      ) {
        return component
      }
    }

    // Check wires
    for (const wire of wires.value) {
      for (let i = 0; i < wire.points.length - 1; i++) {
        const p1 = wire.points[i]
        const p2 = wire.points[i + 1]

        // Simple distance check from point to line segment
        const dist = distanceToLineSegment({ x, y }, p1, p2)
        if (dist < 5) {
          return wire
        }
      }
    }

    return null
  }

  function distanceToLineSegment(
    point: { x: number; y: number },
    p1: { x: number; y: number },
    p2: { x: number; y: number }
  ): number {
    const A = point.x - p1.x
    const B = point.y - p1.y
    const C = p2.x - p1.x
    const D = p2.y - p1.y

    const dot = A * C + B * D
    const lenSq = C * C + D * D
    let param = -1

    if (lenSq !== 0) {
      param = dot / lenSq
    }

    let xx, yy

    if (param < 0) {
      xx = p1.x
      yy = p1.y
    } else if (param > 1) {
      xx = p2.x
      yy = p2.y
    } else {
      xx = p1.x + param * C
      yy = p1.y + param * D
    }

    const dx = point.x - xx
    const dy = point.y - yy

    return Math.sqrt(dx * dx + dy * dy)
  }

  function handleCanvasClick(e: MouseEvent) {
    const rect = canvasRef.value?.getBoundingClientRect()
    if (!rect) return

    const x = e.clientX - rect.left
    const y = e.clientY - rect.top
    const canvasPoint = canvas.screenToCanvas({ x, y })

    if (currentTool.value === 'select') {
      const element = findElementAt(canvasPoint.x, canvasPoint.y)
      if (element) {
        selectElement(element.id, e.shiftKey)
      } else {
        clearSelection()
      }
    } else if (currentTool.value === 'wire') {
      if (!isDrawingWire.value) {
        startWire(canvasPoint.x, canvasPoint.y)
      } else {
        if (e.detail === 2) {
          // Double click to finish wire
          finishWire()
        } else {
          addWirePoint(canvasPoint.x, canvasPoint.y)
        }
      }
    } else if (currentTool.value === 'component') {
      const component = addComponent('IC', canvasPoint.x, canvasPoint.y)
      selectElement(component.id)
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Delete' || e.key === 'Backspace') {
      deleteSelected()
    } else if (e.key === 'Escape') {
      if (isDrawingWire.value) {
        currentWire.value = null
        isDrawingWire.value = false
        redraw()
      } else {
        clearSelection()
      }
    }
  }

  // Setup event listeners
  if (canvasRef.value) {
    canvasRef.value.addEventListener('click', handleCanvasClick)
    window.addEventListener('keydown', handleKeyDown)
  }

  // Cleanup on unmount
  onUnmounted(() => {
    if (canvasRef.value) {
      canvasRef.value.removeEventListener('click', handleCanvasClick)
    }
    window.removeEventListener('keydown', handleKeyDown)
    
    // Cancel any pending animation frame
    if (rafId !== null) {
      cancelAnimationFrame(rafId)
    }
  })

  // Initial draw
  redraw()

  return {
    ...canvas,
    components,
    wires,
    selectedElements,
    selectedComponent,
    selectedWire,
    selectedAnnotation,
    currentTool,
    isDrawingWire,
    currentWire,
    addComponent,
    addComponentFromLibrary,
    startWire,
    finishWire,
    addWirePoint,
    addWire,
    selectElement,
    clearSelection,
    deleteSelected,
    findElementAt,
    redraw
  }
}