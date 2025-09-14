import { ref, computed, type Ref } from 'vue'
import type { Point, SchematicComponent, SchematicWire } from '@/types/schematic'

// 网格吸附配置
const GRID_SIZE = 10
const SNAP_THRESHOLD = 15 // 吸附距离阈值
const CONNECTION_HIGHLIGHT_RADIUS = 8

interface WiringState {
  isWiring: boolean
  currentWire: SchematicWire | null
  startPoint: Point | null
  previewPoint: Point | null
  snapPoint: Point | null
  nearestPin: PinInfo | null
  orthogonalMode: boolean
}

interface PinInfo {
  componentId: string
  pinId: string
  position: Point
  isConnected: boolean
}

interface ConnectionPoint {
  point: Point
  type: 'pin' | 'wire' | 'junction'
  elementId: string
  pinId?: string
}

export function useSmartWiring(
  components: Ref<SchematicComponent[]>,
  wires: Ref<SchematicWire[]>
) {
  // 连线状态
  const state = ref<WiringState>({
    isWiring: false,
    currentWire: null,
    startPoint: null,
    previewPoint: null,
    snapPoint: null,
    nearestPin: null,
    orthogonalMode: true // 默认开启正交走线
  })

  // 辅助线显示
  const guidelines = ref<Array<{ x1: number; y1: number; x2: number; y2: number }>>([])
  
  // 高亮的连接点
  const highlightedPoints = ref<ConnectionPoint[]>([])

  // 网格吸附
  function snapToGrid(point: Point): Point {
    return {
      x: Math.round(point.x / GRID_SIZE) * GRID_SIZE,
      y: Math.round(point.y / GRID_SIZE) * GRID_SIZE
    }
  }

  // 查找最近的引脚
  function findNearestPin(point: Point): PinInfo | null {
    let nearestPin: PinInfo | null = null
    let minDistance = SNAP_THRESHOLD

    for (const component of components.value) {
      for (const pin of component.pins || []) {
        const pinPos = {
          x: component.x + pin.x,
          y: component.y + pin.y
        }
        const distance = Math.sqrt(
          Math.pow(point.x - pinPos.x, 2) + 
          Math.pow(point.y - pinPos.y, 2)
        )
        
        if (distance < minDistance) {
          minDistance = distance
          nearestPin = {
            componentId: component.id,
            pinId: pin.id,
            position: pinPos,
            isConnected: checkPinConnection(component.id, pin.id)
          }
        }
      }
    }

    return nearestPin
  }

  // 查找最近的连线点
  function findNearestWirePoint(point: Point): Point | null {
    let nearestPoint: Point | null = null
    let minDistance = SNAP_THRESHOLD

    for (const wire of wires.value) {
      // 检查线段端点
      for (const wirePoint of wire.points) {
        const distance = Math.sqrt(
          Math.pow(point.x - wirePoint.x, 2) + 
          Math.pow(point.y - wirePoint.y, 2)
        )
        
        if (distance < minDistance) {
          minDistance = distance
          nearestPoint = wirePoint
        }
      }

      // 检查线段上的点
      for (let i = 0; i < wire.points.length - 1; i++) {
        const p1 = wire.points[i]
        const p2 = wire.points[i + 1]
        const nearestOnSegment = nearestPointOnSegment(point, p1, p2)
        
        const distance = Math.sqrt(
          Math.pow(point.x - nearestOnSegment.x, 2) + 
          Math.pow(point.y - nearestOnSegment.y, 2)
        )
        
        if (distance < minDistance) {
          minDistance = distance
          nearestPoint = nearestOnSegment
        }
      }
    }

    return nearestPoint
  }

  // 计算线段上最近的点
  function nearestPointOnSegment(point: Point, p1: Point, p2: Point): Point {
    const dx = p2.x - p1.x
    const dy = p2.y - p1.y
    const length = Math.sqrt(dx * dx + dy * dy)
    
    if (length === 0) return p1
    
    const t = Math.max(0, Math.min(1, 
      ((point.x - p1.x) * dx + (point.y - p1.y) * dy) / (length * length)
    ))
    
    return {
      x: p1.x + t * dx,
      y: p1.y + t * dy
    }
  }

  // 检查引脚是否已连接
  function checkPinConnection(componentId: string, pinId: string): boolean {
    return wires.value.some(wire => 
      wire.connections?.some(conn => 
        conn.componentId === componentId && conn.pinId === pinId
      )
    )
  }

  // 生成正交路径
  function generateOrthogonalPath(start: Point, end: Point): Point[] {
    const points: Point[] = [start]
    
    // 判断主要方向
    const dx = Math.abs(end.x - start.x)
    const dy = Math.abs(end.y - start.y)
    
    if (dx > dy) {
      // 水平优先
      if (start.x !== end.x) {
        points.push({ x: end.x, y: start.y })
      }
      if (start.y !== end.y) {
        points.push(end)
      }
    } else {
      // 垂直优先
      if (start.y !== end.y) {
        points.push({ x: start.x, y: end.y })
      }
      if (start.x !== end.x) {
        points.push(end)
      }
    }
    
    // 去除重复点
    return points.filter((point, index) => 
      index === 0 || (point.x !== points[index - 1].x || point.y !== points[index - 1].y)
    )
  }

  // 开始连线
  function startWiring(point: Point) {
    // 查找吸附点
    const pin = findNearestPin(point)
    const wirePoint = findNearestWirePoint(point)
    
    let startPoint: Point
    
    if (pin) {
      startPoint = pin.position
      highlightedPoints.value = [{
        point: pin.position,
        type: 'pin',
        elementId: pin.componentId,
        pinId: pin.pinId
      }]
    } else if (wirePoint) {
      startPoint = wirePoint
      highlightedPoints.value = [{
        point: wirePoint,
        type: 'wire',
        elementId: 'wire'
      }]
    } else {
      startPoint = snapToGrid(point)
      highlightedPoints.value = []
    }
    
    state.value = {
      isWiring: true,
      currentWire: {
        id: `wire_${Date.now()}`,
        points: [startPoint],
        width: 2,
        color: '#0066cc',
        selected: false,
        connections: pin ? [{
          componentId: pin.componentId,
          pinId: pin.pinId,
          point: pin.position
        }] : []
      },
      startPoint,
      previewPoint: null,
      snapPoint: null,
      nearestPin: pin,
      orthogonalMode: true
    }
  }

  // 更新连线预览
  function updateWiring(point: Point) {
    if (!state.value.isWiring || !state.value.currentWire) return
    
    // 查找吸附点
    const pin = findNearestPin(point)
    const wirePoint = findNearestWirePoint(point)
    
    let targetPoint: Point
    
    if (pin) {
      targetPoint = pin.position
      state.value.snapPoint = pin.position
      state.value.nearestPin = pin
      
      // 更新高亮点
      highlightedPoints.value = [
        ...highlightedPoints.value.filter(p => p.type !== 'pin' || p.elementId !== pin.componentId),
        {
          point: pin.position,
          type: 'pin',
          elementId: pin.componentId,
          pinId: pin.pinId
        }
      ]
    } else if (wirePoint) {
      targetPoint = wirePoint
      state.value.snapPoint = wirePoint
      state.value.nearestPin = null
      
      highlightedPoints.value = [
        ...highlightedPoints.value.filter(p => p.type !== 'wire'),
        {
          point: wirePoint,
          type: 'wire',
          elementId: 'wire'
        }
      ]
    } else {
      targetPoint = snapToGrid(point)
      state.value.snapPoint = null
      state.value.nearestPin = null
      
      // 只保留非临时高亮点
      highlightedPoints.value = highlightedPoints.value.filter(
        p => p.type === 'pin' && state.value.currentWire?.connections?.some(
          c => c.componentId === p.elementId && c.pinId === p.pinId
        )
      )
    }
    
    state.value.previewPoint = targetPoint
    
    // 生成预览路径
    if (state.value.orthogonalMode && state.value.startPoint) {
      const lastPoint = state.value.currentWire.points[state.value.currentWire.points.length - 1]
      const orthogonalPoints = generateOrthogonalPath(lastPoint, targetPoint)
      
      // 更新辅助线
      guidelines.value = []
      for (let i = 0; i < orthogonalPoints.length - 1; i++) {
        guidelines.value.push({
          x1: orthogonalPoints[i].x,
          y1: orthogonalPoints[i].y,
          x2: orthogonalPoints[i + 1].x,
          y2: orthogonalPoints[i + 1].y
        })
      }
    } else {
      guidelines.value = []
    }
  }

  // 添加连线点
  function addWirePoint(point: Point) {
    if (!state.value.isWiring || !state.value.currentWire) return
    
    const pin = findNearestPin(point)
    const wirePoint = findNearestWirePoint(point)
    
    let targetPoint: Point
    
    if (pin) {
      targetPoint = pin.position
      
      // 添加连接信息
      if (!state.value.currentWire.connections) {
        state.value.currentWire.connections = []
      }
      state.value.currentWire.connections.push({
        componentId: pin.componentId,
        pinId: pin.pinId,
        point: pin.position
      })
    } else if (wirePoint) {
      targetPoint = wirePoint
    } else {
      targetPoint = snapToGrid(point)
    }
    
    // 生成正交路径并添加到连线
    if (state.value.orthogonalMode) {
      const lastPoint = state.value.currentWire.points[state.value.currentWire.points.length - 1]
      const orthogonalPoints = generateOrthogonalPath(lastPoint, targetPoint)
      
      // 添加中间点（跳过第一个点因为它已经存在）
      for (let i = 1; i < orthogonalPoints.length; i++) {
        state.value.currentWire.points.push(orthogonalPoints[i])
      }
    } else {
      state.value.currentWire.points.push(targetPoint)
    }
    
    // 如果连接到引脚，结束连线
    if (pin && state.value.currentWire.connections && state.value.currentWire.connections.length >= 2) {
      finishWiring()
    }
  }

  // 完成连线
  function finishWiring() {
    if (!state.value.isWiring || !state.value.currentWire) return
    
    // 至少需要两个点才能创建连线
    if (state.value.currentWire.points.length >= 2) {
      wires.value.push(state.value.currentWire)
    }
    
    // 重置状态
    state.value = {
      isWiring: false,
      currentWire: null,
      startPoint: null,
      previewPoint: null,
      snapPoint: null,
      nearestPin: null,
      orthogonalMode: true
    }
    
    guidelines.value = []
    highlightedPoints.value = []
  }

  // 取消连线
  function cancelWiring() {
    state.value = {
      isWiring: false,
      currentWire: null,
      startPoint: null,
      previewPoint: null,
      snapPoint: null,
      nearestPin: null,
      orthogonalMode: true
    }
    
    guidelines.value = []
    highlightedPoints.value = []
  }

  // 切换正交模式
  function toggleOrthogonalMode() {
    state.value.orthogonalMode = !state.value.orthogonalMode
  }

  // 删除连线
  function deleteWire(wireId: string) {
    const index = wires.value.findIndex(w => w.id === wireId)
    if (index !== -1) {
      wires.value.splice(index, 1)
    }
  }

  // 编辑连线点
  function editWirePoint(wireId: string, pointIndex: number, newPosition: Point) {
    const wire = wires.value.find(w => w.id === wireId)
    if (wire && pointIndex >= 0 && pointIndex < wire.points.length) {
      const snappedPoint = snapToGrid(newPosition)
      wire.points[pointIndex] = snappedPoint
    }
  }

  // 自动路由（简化版）
  function autoRoute(start: Point, end: Point): Point[] {
    // 简单的L型路由
    const points: Point[] = [start]
    
    // 检查是否需要避开组件
    const needsAvoidance = checkObstacles(start, end)
    
    if (needsAvoidance) {
      // 添加绕行点
      const midPoint = {
        x: start.x,
        y: end.y - 50 // 简单的向上绕行
      }
      points.push(midPoint)
      points.push({ x: end.x, y: midPoint.y })
    } else {
      // 直接L型连接
      if (Math.abs(end.x - start.x) > Math.abs(end.y - start.y)) {
        points.push({ x: end.x, y: start.y })
      } else {
        points.push({ x: start.x, y: end.y })
      }
    }
    
    points.push(end)
    return points
  }

  // 检查路径上是否有障碍物
  function checkObstacles(start: Point, end: Point): boolean {
    // 简化版：检查路径是否穿过组件
    for (const component of components.value) {
      const bounds = {
        left: component.x - 50,
        right: component.x + 50,
        top: component.y - 30,
        bottom: component.y + 30
      }
      
      // 检查线段是否与组件边界相交
      if (lineIntersectsRect(start, end, bounds)) {
        return true
      }
    }
    
    return false
  }

  // 检查线段是否与矩形相交
  function lineIntersectsRect(
    p1: Point, 
    p2: Point, 
    rect: { left: number; right: number; top: number; bottom: number }
  ): boolean {
    // 简化检查：如果线段的两个端点都在矩形外的同一侧，则不相交
    if ((p1.x < rect.left && p2.x < rect.left) ||
        (p1.x > rect.right && p2.x > rect.right) ||
        (p1.y < rect.top && p2.y < rect.top) ||
        (p1.y > rect.bottom && p2.y > rect.bottom)) {
      return false
    }
    
    // 更详细的相交检查可以后续添加
    return true
  }

  // 渲染辅助元素
  function renderHelpers(ctx: CanvasRenderingContext2D) {
    // 渲染辅助线
    if (guidelines.value.length > 0) {
      ctx.save()
      ctx.strokeStyle = '#00ff00'
      ctx.lineWidth = 1
      ctx.setLineDash([5, 5])
      
      guidelines.value.forEach(line => {
        ctx.beginPath()
        ctx.moveTo(line.x1, line.y1)
        ctx.lineTo(line.x2, line.y2)
        ctx.stroke()
      })
      
      ctx.restore()
    }
    
    // 渲染高亮点
    highlightedPoints.value.forEach(point => {
      ctx.save()
      ctx.fillStyle = point.type === 'pin' ? '#ff0000' : '#0080ff'
      ctx.strokeStyle = '#ffffff'
      ctx.lineWidth = 2
      
      ctx.beginPath()
      ctx.arc(point.point.x, point.point.y, CONNECTION_HIGHLIGHT_RADIUS, 0, Math.PI * 2)
      ctx.stroke()
      ctx.fill()
      
      ctx.restore()
    })
    
    // 渲染当前连线预览
    if (state.value.isWiring && state.value.currentWire && state.value.previewPoint) {
      ctx.save()
      ctx.strokeStyle = '#0066cc'
      ctx.lineWidth = 2
      ctx.setLineDash([10, 5])
      
      const lastPoint = state.value.currentWire.points[state.value.currentWire.points.length - 1]
      
      if (state.value.orthogonalMode) {
        const orthogonalPoints = generateOrthogonalPath(lastPoint, state.value.previewPoint)
        
        ctx.beginPath()
        ctx.moveTo(orthogonalPoints[0].x, orthogonalPoints[0].y)
        for (let i = 1; i < orthogonalPoints.length; i++) {
          ctx.lineTo(orthogonalPoints[i].x, orthogonalPoints[i].y)
        }
        ctx.stroke()
      } else {
        ctx.beginPath()
        ctx.moveTo(lastPoint.x, lastPoint.y)
        ctx.lineTo(state.value.previewPoint.x, state.value.previewPoint.y)
        ctx.stroke()
      }
      
      ctx.restore()
    }
  }

  return {
    // 状态
    state: computed(() => state.value),
    guidelines: computed(() => guidelines.value),
    highlightedPoints: computed(() => highlightedPoints.value),
    
    // 兼容性属性
    highlightedPoint: computed(() => highlightedPoints.value[0] || null),
    isDrawing: computed(() => state.value.isWiring),
    previewPath: computed(() => state.value.previewPoint ? [state.value.previewPoint] : []),
    currentWire: computed(() => state.value.currentWire),
    
    // 方法
    startWiring,
    updateWiring,
    addWirePoint,
    finishWiring,
    cancelWiring,
    toggleOrthogonalMode,
    deleteWire,
    editWirePoint,
    autoRoute,
    renderHelpers,
    
    // 工具函数
    snapToGrid,
    findNearestPin,
    findNearestWirePoint
  }
}