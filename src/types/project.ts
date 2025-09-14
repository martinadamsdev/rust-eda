export interface Project {
  id: string
  name: string
  path: string
  createdAt: Date
  modifiedAt: Date
  version: string
  description?: string
  schematic: SchematicData
  pcb: PCBData
  libraries: LibraryData[]
  settings: ProjectSettings
}

export interface SchematicData {
  sheets: SchematicSheet[]
  components: Component[]
  wires: Wire[]
  nets: Net[]
  buses: Bus[]
}

export interface PCBData {
  layers: PCBLayer[]
  components: PCBComponent[]
  traces: Trace[]
  vias: Via[]
  pads: Pad[]
  keepouts: Keepout[]
  dimensions: BoardDimensions
}

export interface LibraryData {
  id: string
  name: string
  components: LibraryComponent[]
  footprints: Footprint[]
}

export interface ProjectSettings {
  units: 'mm' | 'mil' | 'inch'
  gridSize: number
  snapToGrid: boolean
  autoSave: boolean
  autoSaveInterval: number
}

export interface SchematicSheet {
  id: string
  name: string
  size: SheetSize
  components: string[]
}

export interface Component {
  id: string
  name: string
  reference: string
  value: string
  type: ComponentType
  x: number
  y: number
  rotation: number
  visible: boolean
  locked: boolean
  footprint: string
  datasheet?: string
  description?: string
  pins: Pin[]
}

export interface Wire {
  id: string
  startX: number
  startY: number
  endX: number
  endY: number
  netId?: string
  width: number
  color?: string
}

export interface Net {
  id: string
  name: string
  wires: string[]
  pins: string[]
}

export interface Bus {
  id: string
  name: string
  nets: string[]
}

export interface PCBLayer {
  id: string
  name: string
  type: LayerType
  visible: boolean
  color: string
  order: number
}

export interface PCBComponent {
  id: string
  schematicRef: string
  footprint: string
  x: number
  y: number
  rotation: number
  side: 'top' | 'bottom'
  locked: boolean
}

export interface Trace {
  id: string
  netId: string
  layer: string
  width: number
  points: Point[]
}

export interface Via {
  id: string
  netId: string
  x: number
  y: number
  diameter: number
  drill: number
  layers: string[]
}

export interface Pad {
  id: string
  componentId: string
  pinNumber: string
  netId?: string
  x: number
  y: number
  shape: PadShape
  size: Size
  drill?: number
  layer: string
}

export interface Keepout {
  id: string
  layer: string
  type: 'trace' | 'via' | 'component'
  polygon: Point[]
}

export interface BoardDimensions {
  width: number
  height: number
  thickness: number
  origin: Point
}

export interface LibraryComponent {
  id: string
  name: string
  reference: string
  value: string
  symbol: Symbol
  footprints: string[]
  datasheet?: string
  description?: string
  keywords?: string[]
}

export interface Footprint {
  id: string
  name: string
  pads: FootprintPad[]
  silkscreen: GraphicElement[]
  courtyard: GraphicElement[]
  fabrication: GraphicElement[]
}

export interface Pin {
  id: string
  number: string
  name: string
  type: PinType
  x: number
  y: number
  length: number
  orientation: PinOrientation
}

export interface Point {
  x: number
  y: number
}

export interface Size {
  width: number
  height: number
}

export interface Symbol {
  id: string
  name: string
  pins: Pin[]
  graphics: GraphicElement[]
}

export interface GraphicElement {
  type: 'line' | 'arc' | 'circle' | 'rectangle' | 'polygon' | 'text'
  layer: string
  points?: Point[]
  center?: Point
  radius?: number
  startAngle?: number
  endAngle?: number
  text?: string
  fontSize?: number
  width?: number
  fill?: boolean
  color?: string
}

export interface FootprintPad {
  number: string
  type: 'smd' | 'through' | 'connect'
  shape: PadShape
  x: number
  y: number
  size: Size
  drill?: number
  layers: string[]
}

export type ComponentType =
  | 'resistor'
  | 'capacitor'
  | 'inductor'
  | 'diode'
  | 'transistor'
  | 'ic'
  | 'connector'
  | 'switch'
  | 'crystal'
  | 'power'
  | 'ground'
  | 'other'

export type SheetSize = 'A4' | 'A3' | 'A2' | 'A1' | 'A0' | 'Letter' | 'Legal' | 'Custom'

export type LayerType =
  | 'signal'
  | 'power'
  | 'mixed'
  | 'silkscreen'
  | 'soldermask'
  | 'paste'
  | 'mechanical'
  | 'keepout'

export type PadShape = 'circle' | 'rectangle' | 'oval' | 'roundrect' | 'custom'

export type PinType =
  | 'input'
  | 'output'
  | 'bidirectional'
  | 'power'
  | 'ground'
  | 'passive'
  | 'no_connect'
  | 'open_collector'
  | 'open_emitter'
  | 'unspecified'

export type PinOrientation = 'up' | 'down' | 'left' | 'right'

export interface ProjectInfo {
  id: string
  name: string
  path: string
  modifiedAt: Date
  thumbnail?: string
}
