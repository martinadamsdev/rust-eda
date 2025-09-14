<template>
  <div class="schematic-editor">
    <n-layout has-sider>
      <n-layout-sider
        bordered
        show-trigger
        collapse-mode="width"
        :collapsed-width="64"
        :width="320"
        :native-scrollbar="false"
      >
        <ComponentLibrary
          @select-component="handleSelectComponent"
          @add-component="handleAddComponent"
        />
      </n-layout-sider>
      
      <n-layout-content>
        <div class="editor-container">
          <div class="editor-toolbar">
            <n-space>
              <n-button-group>
                <n-button
                  v-for="tool in tools"
                  :key="tool.name"
                  :type="currentTool === tool.name ? 'primary' : 'default'"
                  @click="setTool(tool.name as any)"
                >
                  <template #icon>
                    <n-icon :component="tool.icon" />
                  </template>
                  {{ tool.label }}
                </n-button>
              </n-button-group>
              <n-divider vertical />
              <n-button :disabled="!hasSelection" @click="editor?.deleteSelected()">
                Delete Selected
              </n-button>
              <n-button @click="editor?.clearSelection()">Clear Selection</n-button>
              <n-divider vertical />
              <span class="status-text">
                Position: ({{ Math.round(mousePos.x) }}, {{ Math.round(mousePos.y) }}) | Zoom:
                {{ Math.round(zoom * 100) }}%
              </span>
            </n-space>
          </div>
          <canvas
            ref="canvasRef"
            class="editor-canvas"
            tabindex="0"
            role="application"
            :aria-label="`Schematic editor canvas with ${editor?.components.value.length || 0} components and ${editor?.wires.value.length || 0} wires`"
            aria-describedby="canvas-instructions"
            @click="handleCanvasClick"
            @dblclick="handleCanvasDoubleClick"
            @keydown="handleKeyDown"
            @dragover="handleDragOver"
            @drop="handleDrop"
          />
          <div id="canvas-instructions" class="sr-only">
            Use arrow keys to navigate, Enter to select, Space to place components.
            Press Tab to cycle through components. Press Escape to cancel operations.
          </div>
        </div>
      </n-layout-content>
      
      <n-layout-sider
        bordered
        show-trigger
        collapse-mode="width"
        :collapsed-width="64"
        :width="360"
        :native-scrollbar="false"
        position="right"
      >
        <PropertyPanel
          :selected-component="editor?.selectedComponent?.value"
          :selected-wire="editor?.selectedWire?.value"
          :selected-annotation="editor?.selectedAnnotation?.value"
          @update-component="handleUpdateComponent"
          @update-wire="handleUpdateWire"
          @update-annotation="handleUpdateAnnotation"
          @update-project="handleUpdateProject"
        />
      </n-layout-sider>
    </n-layout>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { 
  NLayout, 
  NLayoutSider, 
  NLayoutContent,
  NSpace, 
  NButton, 
  NButtonGroup, 
  NDivider, 
  NIcon,
  useMessage
} from 'naive-ui'
import { HandLeft, GitBranch, HardwareChip, Move, Trash } from '@vicons/ionicons5'
import ComponentLibrary from '@/components/library/ComponentLibrary.vue'
import PropertyPanel from '@/components/PropertyPanel.vue'
import { useSchematicEditor } from '@/composables/useSchematicEditor'
import { useSmartWiring } from '@/composables/useSmartWiring'
import type { ComponentSymbol } from '@/types/library'
import { snapToGrid } from '@/utils/helpers'

const message = useMessage()
const canvasRef = ref<HTMLCanvasElement | null>(null)
let editor: ReturnType<typeof useSchematicEditor> | null = null
let smartWiring: ReturnType<typeof useSmartWiring> | null = null

const tools = [
  { name: 'select', label: 'Select', icon: HandLeft },
  { name: 'wire', label: 'Wire', icon: GitBranch },
  { name: 'component', label: 'Component', icon: HardwareChip },
  { name: 'move', label: 'Move', icon: Move },
  { name: 'delete', label: 'Delete', icon: Trash }
]

const currentTool = ref<'select' | 'wire' | 'component' | 'move' | 'delete'>('select')
const mousePos = ref({ x: 0, y: 0 })
const zoom = ref(1)
const hasSelection = ref(false)
const selectedLibraryComponent = ref<ComponentSymbol | null>(null)

function setTool(tool: 'select' | 'wire' | 'component' | 'move' | 'delete') {
  currentTool.value = tool
  if (editor) {
    editor.currentTool.value = tool
  }
}

function handleCanvasClick(e: MouseEvent) {
  if (!canvasRef.value || !editor) return

  const rect = canvasRef.value.getBoundingClientRect()
  const x = e.clientX - rect.left
  const y = e.clientY - rect.top
  const canvasPoint = editor.screenToCanvas({ x, y })

  if (currentTool.value === 'select') {
    const element = editor.findElementAt(canvasPoint.x, canvasPoint.y)
    if (element) {
      editor.selectElement(element.id, e.shiftKey)
    } else {
      editor.clearSelection()
    }
  } else if (currentTool.value === 'wire' && smartWiring) {
    if (!smartWiring.isDrawing.value) {
      // Find nearest connection point
      const connectionPoint = smartWiring.findNearestConnectionPoint(
        canvasPoint,
        [...editor.components.value]
      )
      
      if (connectionPoint) {
        smartWiring.startWiring(connectionPoint.point, connectionPoint.element)
      } else {
        smartWiring.startWiring(canvasPoint)
      }
    } else {
      // Add wire point with auto-routing
      const connectionPoint = smartWiring.findNearestConnectionPoint(
        canvasPoint,
        [...editor.components.value]
      )
      
      if (connectionPoint) {
        smartWiring.addPoint(connectionPoint.point)
      } else {
        smartWiring.addPoint(canvasPoint)
      }
    }
  } else if (currentTool.value === 'component') {
    // If a library component is selected, place it
    if (selectedLibraryComponent.value) {
      editor.addComponentFromLibrary(
        selectedLibraryComponent.value.id,
        'standard', // TODO: Get actual library ID
        canvasPoint.x,
        canvasPoint.y
      )
    } else {
      const component = editor.addComponent('IC', canvasPoint.x, canvasPoint.y)
      editor.selectElement(component.id)
    }
  }

  hasSelection.value = editor.selectedElements.value.size > 0
}

function handleCanvasDoubleClick(_e: MouseEvent) {
  if (!editor || !smartWiring) return

  if (currentTool.value === 'wire' && smartWiring.isDrawing.value) {
    const wire = smartWiring.finishWiring()
    if (wire) {
      editor.addWire(wire)
    }
  }
}

function handleKeyDown(e: KeyboardEvent) {
  if (!editor) return

  if (e.key === 'Delete' || e.key === 'Backspace') {
    editor.deleteSelected()
    hasSelection.value = false
  } else if (e.key === 'Escape') {
    if (smartWiring && smartWiring.isDrawing.value) {
      smartWiring.cancelWiring()
      editor.redraw()
    } else {
      editor.clearSelection()
      hasSelection.value = false
    }
  } else if (e.key === '1') {
    setTool('select')
  } else if (e.key === '2') {
    setTool('wire')
  } else if (e.key === '3') {
    setTool('component')
  }
}

function handleMouseMove(e: MouseEvent) {
  if (!canvasRef.value || !editor) return

  const rect = canvasRef.value.getBoundingClientRect()
  const x = e.clientX - rect.left
  const y = e.clientY - rect.top
  const canvasPoint = editor.screenToCanvas({ x, y })

  mousePos.value = canvasPoint

  // Update smart wiring preview
  if (smartWiring) {
    if (smartWiring.isDrawing.value) {
      // Find nearest connection point for highlighting
      const connectionPoint = smartWiring.findNearestConnectionPoint(
        canvasPoint,
        [...editor.components.value]
      )
      
      if (connectionPoint) {
        smartWiring.highlightConnectionPoint(connectionPoint.point)
      } else {
        smartWiring.clearHighlight()
      }
      
      // Update preview path
      smartWiring.updatePreviewPath(canvasPoint)
      
      // Trigger redraw to show preview
      editor.redraw()
    } else {
      // Highlight potential connection points when hovering
      const connectionPoint = smartWiring.findNearestConnectionPoint(
        canvasPoint,
        [...editor.components.value],
        20 // Larger threshold for hover detection
      )
      
      if (connectionPoint) {
        smartWiring.highlightConnectionPoint(connectionPoint.point)
        editor.redraw()
      } else if (smartWiring.highlightedPoint.value) {
        smartWiring.clearHighlight()
        editor.redraw()
      }
    }
  }
}

function handleSelectComponent(component: ComponentSymbol) {
  selectedLibraryComponent.value = component
  setTool('component')
}

function handleAddComponent(component: ComponentSymbol) {
  if (!editor || !canvasRef.value) return
  
  // Place component at center of visible canvas
  const rect = canvasRef.value.getBoundingClientRect()
  const centerX = rect.width / 2
  const centerY = rect.height / 2
  const canvasPoint = editor.screenToCanvas({ x: centerX, y: centerY })
  
  editor.addComponentFromLibrary(
    component.id,
    'standard', // TODO: Get actual library ID
    canvasPoint.x,
    canvasPoint.y
  )
}

function handleDragOver(e: DragEvent) {
  e.preventDefault()
  if (e.dataTransfer) {
    e.dataTransfer.dropEffect = 'copy'
  }
}

function handleDrop(e: DragEvent) {
  e.preventDefault()
  if (!editor || !e.dataTransfer) return

  try {
    const dataText = e.dataTransfer.getData('application/json')
    if (!dataText) {
      console.warn('No data in drop event')
      return
    }
    
    const data = JSON.parse(dataText)
    if (data.type === 'library-component' || data.type === 'component') {
      const rect = canvasRef.value?.getBoundingClientRect()
      if (!rect) {
        console.error('Canvas element not found')
        return
      }
      
      const x = e.clientX - rect.left
      const y = e.clientY - rect.top
      const canvasPoint = editor.screenToCanvas({ x, y })
      
      const component = editor.addComponentFromLibrary(
        data.componentId,
        data.libraryId,
        canvasPoint.x,
        canvasPoint.y
      )
      
      if (!component) {
        message.error('Failed to add component to schematic')
      }
    }
  } catch (error) {
    console.error('Failed to handle drop:', error)
    message.error('Failed to drop component. Please try again.')
  }
}

onMounted(() => {
  if (canvasRef.value) {
    editor = useSchematicEditor(canvasRef)
    smartWiring = useSmartWiring()

    // Add some demo components from library
    const libraryStore = useLibraryStore()
    const resistor = libraryStore.getComponent('std_resistor')
    const capacitor = libraryStore.getComponent('std_capacitor')
    const opamp = libraryStore.getComponent('std_opamp')
    
    if (resistor) {
      editor.addComponentFromLibrary('std_resistor', 'standard', 200, 200)
    }
    if (capacitor) {
      editor.addComponentFromLibrary('std_capacitor', 'standard', 400, 200)
    }
    if (opamp) {
      editor.addComponentFromLibrary('std_opamp', 'standard', 300, 350)
    }

    canvasRef.value.addEventListener('mousemove', handleMouseMove)
    
    // Set up smart wiring custom render function
    if (editor && smartWiring) {
      const originalRedraw = editor.redraw.bind(editor)
      editor.redraw = () => {
        originalRedraw()
        
        // Draw smart wiring overlays
        const ctx = canvasRef.value?.getContext('2d')
        if (!ctx) return
        
        // Draw highlighted connection point
        if (smartWiring.highlightedPoint.value) {
          const pt = smartWiring.highlightedPoint.value
          ctx.save()
          ctx.strokeStyle = '#4CAF50'
          ctx.fillStyle = 'rgba(76, 175, 80, 0.2)'
          ctx.lineWidth = 2
          ctx.beginPath()
          ctx.arc(pt.x, pt.y, 8, 0, Math.PI * 2)
          ctx.fill()
          ctx.stroke()
          
          // Draw snap indicator
          ctx.strokeStyle = '#4CAF50'
          ctx.lineWidth = 1
          ctx.setLineDash([2, 2])
          ctx.beginPath()
          ctx.arc(pt.x, pt.y, 15, 0, Math.PI * 2)
          ctx.stroke()
          ctx.setLineDash([])
          ctx.restore()
        }
        
        // Draw preview wire path
        if (smartWiring.isDrawing.value && smartWiring.previewPath.value.length > 0) {
          const path = smartWiring.previewPath.value
          ctx.save()
          ctx.strokeStyle = 'rgba(33, 150, 243, 0.6)'
          ctx.lineWidth = 2
          ctx.setLineDash([5, 5])
          ctx.beginPath()
          ctx.moveTo(path[0].x, path[0].y)
          for (let i = 1; i < path.length; i++) {
            ctx.lineTo(path[i].x, path[i].y)
          }
          ctx.stroke()
          ctx.setLineDash([])
          ctx.restore()
        }
        
        // Draw current wire being drawn
        if (smartWiring.isDrawing.value && smartWiring.currentWire.value) {
          const wire = smartWiring.currentWire.value
          ctx.save()
          ctx.strokeStyle = '#2196F3'
          ctx.lineWidth = 2
          ctx.beginPath()
          ctx.moveTo(wire.points[0].x, wire.points[0].y)
          for (let i = 1; i < wire.points.length; i++) {
            ctx.lineTo(wire.points[i].x, wire.points[i].y)
          }
          ctx.stroke()
          
          // Draw connection points
          ctx.fillStyle = '#2196F3'
          for (const point of wire.points) {
            ctx.beginPath()
            ctx.arc(point.x, point.y, 3, 0, Math.PI * 2)
            ctx.fill()
          }
          ctx.restore()
        }
      }
    }

    zoom.value = editor.scale.value
  }
})

onUnmounted(() => {
  if (canvasRef.value) {
    canvasRef.value.removeEventListener('mousemove', handleMouseMove)
  }
})

// Property update handlers
function handleUpdateComponent(id: string, property: string, value: any) {
  if (!editor) return
  const component = editor.components.value.find(c => c.id === id)
  if (component) {
    ;(component as any)[property] = value
    editor.redraw()
  }
}

function handleUpdateWire(id: string, property: string, value: any) {
  if (!editor) return
  const wire = editor.wires.value.find(w => w.id === id)
  if (wire) {
    ;(wire as any)[property] = value
    editor.redraw()
  }
}

function handleUpdateAnnotation(id: string, property: string, value: any) {
  // Implement annotation update logic
  if (!editor) return
  // TODO: Add annotation support to editor
  editor.redraw()
}

function handleUpdateProject(property: string, value: any) {
  // Implement project update logic
  message.info(`Project ${property} updated to ${value}`)
}

// Import library store for demo components
import { useLibraryStore } from '@/stores/library'
</script>

<style scoped>
.schematic-editor {
  width: 100%;
  height: 100%;
}

.editor-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  position: relative;
}

.editor-toolbar {
  position: absolute;
  top: 10px;
  left: 10px;
  right: 10px;
  z-index: 10;
  background: rgba(255, 255, 255, 0.95);
  border-radius: 4px;
  padding: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.editor-canvas {
  width: 100%;
  height: 100%;
  background: #f9f9f9;
  cursor: crosshair;
  outline: none;
}

.editor-canvas:focus {
  box-shadow: inset 0 0 0 2px var(--primary-color);
}

/* Accessibility styles */
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}

/* High contrast mode support */
@media (prefers-contrast: high) {
  .editor-canvas {
    border: 2px solid currentColor;
  }
  
  .editor-toolbar {
    border: 1px solid currentColor;
  }
}

/* Reduced motion support */
@media (prefers-reduced-motion: reduce) {
  * {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
  }
}

.status-text {
  color: var(--text-secondary);
  font-size: 12px;
  font-family: monospace;
  padding: 0 8px;
}

:deep(.n-layout-sider) {
  background: #fafafa;
}
</style>