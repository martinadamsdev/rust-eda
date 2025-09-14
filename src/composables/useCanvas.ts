import { ref, onMounted, onUnmounted } from 'vue'
import type { Ref } from 'vue'

export interface CanvasPoint {
  x: number
  y: number
}

export interface CanvasOptions {
  width?: number
  height?: number
  backgroundColor?: string
  gridSize?: number
  gridColor?: string
  showGrid?: boolean
}

export function useCanvas(canvasRef: Ref<HTMLCanvasElement | null>, options: CanvasOptions = {}) {
  const {
    width = 800,
    height = 600,
    backgroundColor = '#ffffff',
    gridSize = 20,
    gridColor = '#e0e0e0',
    showGrid = true
  } = options

  const ctx = ref<CanvasRenderingContext2D | null>(null)
  const scale = ref(1)
  const offsetX = ref(0)
  const offsetY = ref(0)
  const isDragging = ref(false)
  const dragStart = ref<CanvasPoint>({ x: 0, y: 0 })
  const mousePos = ref<CanvasPoint>({ x: 0, y: 0 })

  function initCanvas() {
    if (!canvasRef.value) return

    const canvas = canvasRef.value
    canvas.width = width
    canvas.height = height

    const context = canvas.getContext('2d')
    if (!context) return

    ctx.value = context
    drawBackground()
    if (showGrid) drawGrid()
  }

  function drawBackground() {
    if (!ctx.value) return

    ctx.value.fillStyle = backgroundColor
    ctx.value.fillRect(0, 0, width, height)
  }

  function drawGrid() {
    if (!ctx.value) return

    ctx.value.strokeStyle = gridColor
    ctx.value.lineWidth = 0.5

    // Draw vertical lines
    for (let x = 0; x <= width; x += gridSize) {
      ctx.value.beginPath()
      ctx.value.moveTo(x, 0)
      ctx.value.lineTo(x, height)
      ctx.value.stroke()
    }

    // Draw horizontal lines
    for (let y = 0; y <= height; y += gridSize) {
      ctx.value.beginPath()
      ctx.value.moveTo(0, y)
      ctx.value.lineTo(width, y)
      ctx.value.stroke()
    }
  }

  function clear() {
    if (!ctx.value) return
    ctx.value.clearRect(0, 0, width, height)
    drawBackground()
    if (showGrid) drawGrid()
  }

  function screenToCanvas(point: CanvasPoint): CanvasPoint {
    return {
      x: (point.x - offsetX.value) / scale.value,
      y: (point.y - offsetY.value) / scale.value
    }
  }

  function canvasToScreen(point: CanvasPoint): CanvasPoint {
    return {
      x: point.x * scale.value + offsetX.value,
      y: point.y * scale.value + offsetY.value
    }
  }

  function zoomIn() {
    scale.value = Math.min(scale.value * 1.2, 5)
    redraw()
  }

  function zoomOut() {
    scale.value = Math.max(scale.value / 1.2, 0.1)
    redraw()
  }

  function resetZoom() {
    scale.value = 1
    offsetX.value = 0
    offsetY.value = 0
    redraw()
  }

  function pan(dx: number, dy: number) {
    offsetX.value += dx
    offsetY.value += dy
    redraw()
  }

  function redraw() {
    clear()
    // Redraw all elements here
  }

  function handleMouseDown(e: MouseEvent) {
    if (!canvasRef.value) return

    const rect = canvasRef.value.getBoundingClientRect()
    const x = e.clientX - rect.left
    const y = e.clientY - rect.top

    if (e.button === 1 || (e.button === 0 && e.shiftKey)) {
      // Middle button or shift+left for panning
      isDragging.value = true
      dragStart.value = { x: e.clientX, y: e.clientY }
      canvasRef.value.style.cursor = 'move'
    }

    mousePos.value = screenToCanvas({ x, y })
  }

  function handleMouseMove(e: MouseEvent) {
    if (!canvasRef.value) return

    const rect = canvasRef.value.getBoundingClientRect()
    const x = e.clientX - rect.left
    const y = e.clientY - rect.top

    if (isDragging.value) {
      const dx = e.clientX - dragStart.value.x
      const dy = e.clientY - dragStart.value.y
      pan(dx, dy)
      dragStart.value = { x: e.clientX, y: e.clientY }
    }

    mousePos.value = screenToCanvas({ x, y })
  }

  function handleMouseUp() {
    if (!canvasRef.value) return

    isDragging.value = false
    canvasRef.value.style.cursor = 'default'
  }

  function handleWheel(e: WheelEvent) {
    e.preventDefault()

    if (e.deltaY < 0) {
      zoomIn()
    } else {
      zoomOut()
    }
  }

  onMounted(() => {
    initCanvas()

    if (canvasRef.value) {
      canvasRef.value.addEventListener('mousedown', handleMouseDown)
      canvasRef.value.addEventListener('mousemove', handleMouseMove)
      canvasRef.value.addEventListener('mouseup', handleMouseUp)
      canvasRef.value.addEventListener('wheel', handleWheel)
      canvasRef.value.addEventListener('contextmenu', e => e.preventDefault())
    }
  })

  onUnmounted(() => {
    if (canvasRef.value) {
      canvasRef.value.removeEventListener('mousedown', handleMouseDown)
      canvasRef.value.removeEventListener('mousemove', handleMouseMove)
      canvasRef.value.removeEventListener('mouseup', handleMouseUp)
      canvasRef.value.removeEventListener('wheel', handleWheel)
    }
  })

  return {
    ctx,
    scale,
    offsetX,
    offsetY,
    mousePos,
    isDragging,
    clear,
    redraw,
    zoomIn,
    zoomOut,
    resetZoom,
    pan,
    screenToCanvas,
    canvasToScreen
  }
}
