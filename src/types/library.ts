// 元件分类
export enum ComponentCategory {
  Passive = 'passive',
  Active = 'active',
  IC = 'ic',
  Connector = 'connector',
  Power = 'power',
  Mechanical = 'mechanical',
  Custom = 'custom'
}

// 引脚类型
export enum PinType {
  Input = 'input',
  Output = 'output',
  BiDirectional = 'bidirectional',
  Power = 'power',
  Ground = 'ground',
  Passive = 'passive',
  NotConnected = 'nc'
}

// 引脚定义
export interface ComponentPin {
  id: string
  name: string
  number: string
  type: PinType
  x: number
  y: number
  rotation?: number
  electrical?: {
    voltage?: number
    current?: number
    impedance?: number
  }
}

// 图形元素类型
export type GraphicElement = 
  | { type: 'line'; x1: number; y1: number; x2: number; y2: number; stroke?: string; strokeWidth?: number }
  | { type: 'rect'; x: number; y: number; width: number; height: number; fill?: string; stroke?: string }
  | { type: 'circle'; cx: number; cy: number; r: number; fill?: string; stroke?: string }
  | { type: 'arc'; cx: number; cy: number; r: number; startAngle: number; endAngle: number; stroke?: string }
  | { type: 'polygon'; points: Array<{ x: number; y: number }>; fill?: string; stroke?: string }
  | { type: 'text'; x: number; y: number; text: string; fontSize?: number; fill?: string }
  | { type: 'path'; d: string; fill?: string; stroke?: string; strokeWidth?: number }

// 元件符号定义
export interface ComponentSymbol {
  id: string
  name: string
  category: ComponentCategory
  subcategory?: string
  description?: string
  manufacturer?: string
  partNumber?: string
  datasheet?: string
  keywords: string[]
  
  // 图形表示
  graphics: {
    symbol: GraphicElement[]
    bounds: {
      x: number
      y: number
      width: number
      height: number
    }
  }
  
  // 引脚定义
  pins: ComponentPin[]
  
  // 默认属性
  defaultProperties?: Record<string, string | number | boolean | null>
  
  // 参数
  parameters?: Array<{
    name: string
    label: string
    type: 'string' | 'number' | 'boolean' | 'select'
    default: string | number | boolean
    options?: Array<{ label: string; value: string | number | boolean }>
    unit?: string
    min?: number
    max?: number
  }>
  
  // SPICE 模型
  spiceModel?: string
  
  // 3D 模型
  model3d?: {
    path: string
    scale?: number
    rotation?: { x: number; y: number; z: number }
    offset?: { x: number; y: number; z: number }
  }
  
  // 封装信息
  footprints?: Array<{
    name: string
    description: string
    recommended?: boolean
  }>
  
  // 元数据
  metadata?: {
    createdAt: Date
    updatedAt: Date
    author?: string
    version?: string
    tags?: string[]
  }
}

// 元件库
export interface ComponentLibrary {
  id: string
  name: string
  description?: string
  version: string
  author?: string
  license?: string
  url?: string
  
  // 元件集合
  components: ComponentSymbol[]
  
  // 库配置
  config?: {
    gridSize?: number
    units?: 'mm' | 'mil' | 'inch'
    defaultLineWidth?: number
    defaultTextSize?: number
  }
  
  // 元数据
  metadata?: {
    createdAt: Date
    updatedAt: Date
    downloadUrl?: string
    dependencies?: string[]
    tags?: string[]
  }
}

// 元件实例（放置在原理图中的元件）
export interface ComponentInstance {
  id: string
  symbolId: string
  libraryId?: string
  
  // 位置和变换
  x: number
  y: number
  rotation: number
  mirrored?: boolean
  
  // 实例属性
  reference: string // 如 R1, C2, U3
  value?: string // 如 10kΩ, 100nF
  footprint?: string
  
  // 覆盖的属性
  properties?: Record<string, string | number | boolean | null>
  
  // 连接信息
  connections?: Array<{
    pinId: string
    netId?: string
    wireId?: string
  }>
  
  // 显示控制
  visible?: boolean
  locked?: boolean
  
  // 用户数据
  userData?: Record<string, string | number | boolean | null>
}

// 搜索结果
export interface ComponentSearchResult {
  component: ComponentSymbol
  library: ComponentLibrary
  relevance: number
}

// 导入导出格式
export interface LibraryExportFormat {
  format: 'json' | 'xml' | 'kicad' | 'eagle' | 'altium'
  version: string
  data: ComponentLibrary | string
}