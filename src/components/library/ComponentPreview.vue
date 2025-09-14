<template>
  <div class="component-preview" :class="{ selected: isSelected }" @click="handleClick">
    <div ref="canvasContainer" class="preview-canvas">
      <canvas ref="canvasRef" />
    </div>
    <div class="component-info">
      <div class="component-name">{{ component.name }}</div>
      <div class="component-category">{{ component.categoryId }}</div>
    </div>
    <div v-if="showFavorite" class="favorite-icon" @click.stop="toggleFavorite">
      <n-icon :component="isFavorite ? StarSharp : StarOutline" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from 'vue'
import { NIcon } from 'naive-ui'
import { StarOutline, StarSharp } from '@vicons/ionicons5'
import type { ComponentTemplate, DrawCommand } from '@/api/library'
import { useLibraryStore } from '@/stores/library'

const props = defineProps<{
  component: ComponentTemplate
  isSelected?: boolean
  showFavorite?: boolean
  scale?: number
}>()

const emit = defineEmits<{
  click: [component: ComponentTemplate]
  startDrag: [component: ComponentTemplate, event: DragEvent]
}>()

const libraryStore = useLibraryStore()
const canvasRef = ref<HTMLCanvasElement | null>(null)
const canvasContainer = ref<HTMLDivElement | null>(null)
let animationFrameId: number | null = null
let isDrawScheduled = false

const isFavorite = computed(() => 
  libraryStore.favoriteComponents.has(props.component.id)
)

function handleClick() {
  emit('click', props.component)
}

function toggleFavorite() {
  libraryStore.toggleFavorite(props.component.id)
}

function scheduleRedraw() {
  if (isDrawScheduled) return
  
  isDrawScheduled = true
  animationFrameId = requestAnimationFrame(() => {
    drawComponent()
    isDrawScheduled = false
    animationFrameId = null
  })
}

function drawComponent() {
  if (!canvasRef.value) return
  
  const canvas = canvasRef.value
  const ctx = canvas.getContext('2d')
  if (!ctx) return

  // Set canvas size
  const containerWidth = canvasContainer.value?.clientWidth || 100
  const containerHeight = canvasContainer.value?.clientHeight || 80
  canvas.width = containerWidth * 2 // For retina display
  canvas.height = containerHeight * 2
  canvas.style.width = `${containerWidth}px`
  canvas.style.height = `${containerHeight}px`
  
  // Clear canvas
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  
  // Check if symbol exists and has draw commands
  const symbol = props.component.symbol
  
  if (!symbol || !symbol.drawCommands) {
    // Draw a placeholder rectangle
    ctx.strokeStyle = '#ff0000'
    ctx.lineWidth = 2
    ctx.strokeRect(canvas.width/2 - 20, canvas.height/2 - 10, 40, 20)
    return
  }
  
  // Scale and center the component
  const scale = Math.min(
    (canvas.width * 0.8) / symbol.width,
    (canvas.height * 0.8) / symbol.height
  ) * (props.scale || 1)
  
  ctx.save()
  ctx.translate(canvas.width / 2, canvas.height / 2)
  ctx.scale(scale, scale)
  
  // Draw each draw command
  symbol.drawCommands.forEach(command => {
    drawCommand(ctx, command)
  })
  
  // Draw pins
  ctx.fillStyle = '#ff0000'
  props.component.pins.forEach(pin => {
    ctx.beginPath()
    ctx.arc(pin.x, pin.y, 2, 0, Math.PI * 2)
    ctx.fill()
  })
  
  ctx.restore()
}

function drawCommand(ctx: CanvasRenderingContext2D, command: DrawCommand) {
  // Apply styles
  if (command.style) {
    ctx.strokeStyle = command.style.strokeColor || '#000000'
    ctx.fillStyle = command.style.fillColor || 'transparent'
    ctx.lineWidth = command.style.strokeWidth || 1
  } else {
    ctx.strokeStyle = '#000000'
    ctx.fillStyle = 'transparent'
    ctx.lineWidth = 1
  }
  
  const p = command.parameters
  
  switch (command.commandType) {
    case 'line':
      ctx.beginPath()
      ctx.moveTo(p[0], p[1])
      ctx.lineTo(p[2], p[3])
      ctx.stroke()
      break
      
    case 'rectangle':
      if (command.style?.fillColor && command.style.fillColor !== 'transparent') {
        ctx.fillRect(p[0], p[1], p[2], p[3])
      }
      ctx.strokeRect(p[0], p[1], p[2], p[3])
      break
      
    case 'circle':
      ctx.beginPath()
      ctx.arc(p[0], p[1], p[2], 0, Math.PI * 2)
      if (command.style?.fillColor && command.style.fillColor !== 'transparent') {
        ctx.fill()
      }
      ctx.stroke()
      break
      
    case 'arc':
      ctx.beginPath()
      const startRad = (p[3] * Math.PI) / 180
      const endRad = (p[4] * Math.PI) / 180
      ctx.arc(p[0], p[1], p[2], startRad, endRad)
      ctx.stroke()
      break
      
    case 'polygon':
      ctx.beginPath()
      for (let i = 0; i < p.length; i += 2) {
        if (i === 0) {
          ctx.moveTo(p[i], p[i + 1])
        } else {
          ctx.lineTo(p[i], p[i + 1])
        }
      }
      ctx.closePath()
      if (command.style?.fillColor && command.style.fillColor !== 'transparent') {
        ctx.fill()
      }
      ctx.stroke()
      break
      
    case 'text':
      ctx.font = `${p[2] || 12}px sans-serif`
      ctx.fillStyle = command.style?.strokeColor || '#000000'
      ctx.textAlign = 'center'
      ctx.textBaseline = 'middle'
      ctx.fillText(String(p[3] || ''), p[0], p[1])
      break
  }
}

// Handle drag and drop
function handleDragStart(event: DragEvent) {
  if (!event.dataTransfer) return
  
  event.dataTransfer.effectAllowed = 'copy'
  event.dataTransfer.setData('application/json', JSON.stringify({
    type: 'library-component',
    componentId: props.component.id,
    libraryId: 'standard', // TODO: Get from props or store
    symbol: props.component
  }))
  
  // Create a drag image from the canvas
  if (canvasRef.value) {
    event.dataTransfer.setDragImage(
      canvasRef.value,
      canvasRef.value.width / 2,
      canvasRef.value.height / 2
    )
  }
  
  emit('startDrag', props.component, event)
  libraryStore.addToRecent(props.component)
}

onMounted(() => {
  drawComponent()
  
  // Make draggable
  if (canvasContainer.value) {
    canvasContainer.value.draggable = true
    canvasContainer.value.addEventListener('dragstart', handleDragStart)
  }
  
  // Add resize listener
  window.addEventListener('resize', scheduleRedraw)
})

onUnmounted(() => {
  // Cancel any pending animation frame
  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId)
  }
  
  // Clean up event listeners to prevent memory leaks
  window.removeEventListener('resize', scheduleRedraw)
  
  if (canvasContainer.value) {
    canvasContainer.value.removeEventListener('dragstart', handleDragStart)
  }
})

watch(() => props.component, () => {
  scheduleRedraw()
})
</script>

<style scoped>
.component-preview {
  display: flex;
  flex-direction: column;
  padding: 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  position: relative;
  background: white;
}

.component-preview:hover {
  border-color: var(--primary-color);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.component-preview.selected {
  border-color: var(--primary-color);
  background: var(--primary-color-suppl);
}

.preview-canvas {
  width: 100%;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 8px;
}

.preview-canvas canvas {
  max-width: 100%;
  max-height: 100%;
}

.component-info {
  text-align: center;
}

.component-name {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-color);
  margin-bottom: 2px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.component-category {
  font-size: 10px;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.favorite-icon {
  position: absolute;
  top: 4px;
  right: 4px;
  padding: 4px;
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.9);
  transition: all 0.2s;
}

.favorite-icon:hover {
  background: rgba(255, 255, 255, 1);
  transform: scale(1.1);
}

.favorite-icon :deep(.n-icon) {
  font-size: 16px;
  color: #ffc107;
}
</style>