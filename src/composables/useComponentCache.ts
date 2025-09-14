import type { ComponentSymbol } from '@/types/library'

interface CacheEntry {
  imageData: ImageData | null
  timestamp: number
  width: number
  height: number
}

const CACHE_EXPIRY = 5 * 60 * 1000 // 5 minutes
const MAX_CACHE_SIZE = 100 // Maximum number of cached components

class ComponentCache {
  private cache = new Map<string, CacheEntry>()
  private accessOrder: string[] = []

  /**
   * Get cached image data for a component
   */
  get(componentId: string, width: number, height: number): ImageData | null {
    const entry = this.cache.get(componentId)
    
    if (!entry) return null
    
    // Check if cache is expired
    if (Date.now() - entry.timestamp > CACHE_EXPIRY) {
      this.cache.delete(componentId)
      this.removeFromAccessOrder(componentId)
      return null
    }
    
    // Check if dimensions match
    if (entry.width !== width || entry.height !== height) {
      return null
    }
    
    // Update access order (LRU)
    this.updateAccessOrder(componentId)
    
    return entry.imageData
  }

  /**
   * Store image data in cache
   */
  set(componentId: string, imageData: ImageData, width: number, height: number): void {
    // Enforce cache size limit
    if (this.cache.size >= MAX_CACHE_SIZE && !this.cache.has(componentId)) {
      this.evictLeastRecentlyUsed()
    }
    
    this.cache.set(componentId, {
      imageData,
      timestamp: Date.now(),
      width,
      height
    })
    
    this.updateAccessOrder(componentId)
  }

  /**
   * Clear specific component from cache
   */
  invalidate(componentId: string): void {
    this.cache.delete(componentId)
    this.removeFromAccessOrder(componentId)
  }

  /**
   * Clear all cached data
   */
  clear(): void {
    this.cache.clear()
    this.accessOrder = []
  }

  /**
   * Get cache size
   */
  size(): number {
    return this.cache.size
  }

  /**
   * Update LRU access order
   */
  private updateAccessOrder(componentId: string): void {
    this.removeFromAccessOrder(componentId)
    this.accessOrder.push(componentId)
  }

  /**
   * Remove from access order array
   */
  private removeFromAccessOrder(componentId: string): void {
    const index = this.accessOrder.indexOf(componentId)
    if (index > -1) {
      this.accessOrder.splice(index, 1)
    }
  }

  /**
   * Evict least recently used item
   */
  private evictLeastRecentlyUsed(): void {
    if (this.accessOrder.length > 0) {
      const lruId = this.accessOrder.shift()
      if (lruId) {
        this.cache.delete(lruId)
      }
    }
  }
}

// Singleton instance
let cacheInstance: ComponentCache | null = null

/**
 * Hook to use component preview cache
 */
export function useComponentCache() {
  if (!cacheInstance) {
    cacheInstance = new ComponentCache()
  }

  return {
    getCache: (componentId: string, width: number, height: number) => 
      cacheInstance!.get(componentId, width, height),
    
    setCache: (componentId: string, imageData: ImageData, width: number, height: number) => 
      cacheInstance!.set(componentId, imageData, width, height),
    
    invalidateCache: (componentId: string) => 
      cacheInstance!.invalidate(componentId),
    
    clearCache: () => 
      cacheInstance!.clear(),
    
    getCacheSize: () => 
      cacheInstance!.size()
  }
}

/**
 * Create offscreen canvas for rendering
 */
export function createOffscreenCanvas(width: number, height: number): OffscreenCanvas | HTMLCanvasElement {
  if (typeof OffscreenCanvas !== 'undefined') {
    return new OffscreenCanvas(width, height)
  } else {
    // Fallback for browsers that don't support OffscreenCanvas
    const canvas = document.createElement('canvas')
    canvas.width = width
    canvas.height = height
    return canvas
  }
}

/**
 * Render component to offscreen canvas
 */
export function renderComponentToCanvas(
  component: ComponentSymbol,
  canvas: OffscreenCanvas | HTMLCanvasElement,
  scale: number = 1
): ImageData | null {
  const ctx = canvas.getContext('2d')
  if (!ctx) return null

  // Clear canvas
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  
  // Calculate scale and center
  const bounds = component.graphics?.bounds || { width: 100, height: 60 }
  const canvasScale = Math.min(
    (canvas.width * 0.8) / bounds.width,
    (canvas.height * 0.8) / bounds.height
  ) * scale
  
  ctx.save()
  ctx.translate(canvas.width / 2, canvas.height / 2)
  ctx.scale(canvasScale, canvasScale)
  
  // Render component graphics
  component.graphics.symbol.forEach(element => {
    renderGraphicElement(ctx, element)
  })
  
  // Render pins
  ctx.fillStyle = '#ff0000'
  component.pins.forEach(pin => {
    ctx.beginPath()
    ctx.arc(pin.x, pin.y, 2, 0, Math.PI * 2)
    ctx.fill()
  })
  
  ctx.restore()
  
  // Get image data
  return ctx.getImageData(0, 0, canvas.width, canvas.height)
}

/**
 * Render a single graphic element
 */
function renderGraphicElement(ctx: CanvasRenderingContext2D | OffscreenCanvasRenderingContext2D, element: any): void {
  ctx.strokeStyle = element.stroke || '#000000'
  ctx.fillStyle = element.fill || 'none'
  ctx.lineWidth = element.strokeWidth || 1
  
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
      element.points.forEach((point: any, index: number) => {
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
}