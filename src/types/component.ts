export interface ComponentInstance {
  id: string
  libraryId: string
  reference: string
  value: string
  x: number
  y: number
  rotation: number
  mirror: boolean
}

export interface ComponentLibrary {
  id: string
  name: string
  version: string
  author?: string
  description?: string
  components: ComponentDefinition[]
}

export interface ComponentDefinition {
  id: string
  name: string
  category: string
  symbol: ComponentSymbol
  footprints: string[]
  parameters: ComponentParameter[]
  spiceModel?: string
}

export interface ComponentSymbol {
  width: number
  height: number
  pins: SymbolPin[]
  graphics: SymbolGraphic[]
}

export interface SymbolPin {
  id: string
  number: string
  name: string
  x: number
  y: number
  length: number
  direction: 'left' | 'right' | 'up' | 'down'
  electricalType: PinElectricalType
}

export interface SymbolGraphic {
  type: 'line' | 'rect' | 'circle' | 'arc' | 'text' | 'polygon'
  points?: number[]
  x?: number
  y?: number
  width?: number
  height?: number
  radius?: number
  startAngle?: number
  endAngle?: number
  text?: string
  fontSize?: number
  strokeWidth?: number
  fill?: string
  stroke?: string
}

export interface ComponentParameter {
  name: string
  value: string | number | boolean
  unit?: string
  visible: boolean
}

export type PinElectricalType =
  | 'input'
  | 'output'
  | 'bidirectional'
  | 'tristate'
  | 'passive'
  | 'power_input'
  | 'power_output'
  | 'unconnected'
  | 'open_collector'
  | 'open_emitter'
  | 'nc'
