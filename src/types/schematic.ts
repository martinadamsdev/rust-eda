export interface SchematicEditor {
  id: string
  activeSheet: string
  zoom: number
  panX: number
  panY: number
  gridVisible: boolean
  gridSize: number
  snapToGrid: boolean
  selection: string[]
  clipboard: SchematicObject[]
}

// Define Point interface first
export interface Point {
  x: number
  y: number
}

export interface SchematicObject {
  id: string
  type: 'component' | 'wire' | 'bus' | 'junction' | 'label' | 'power' | 'ground'
  selected: boolean
  locked: boolean
}

export interface SchematicWire {
  id: string
  points: WirePoint[]
  netName?: string
  width: number
  color: string
  selected: boolean
  connections?: WireConnection[]
}

export interface WireConnection {
  componentId: string
  pinId: string
  point: Point
}

export interface WirePoint {
  x: number
  y: number
  junction?: boolean
}

export interface SchematicBus {
  id: string
  name: string
  signals: string[]
  points: WirePoint[]
  width: number
  color: string
}

export interface SchematicLabel {
  id: string
  text: string
  x: number
  y: number
  rotation: number
  fontSize: number
  netName?: string
}

export interface SchematicJunction {
  id: string
  x: number
  y: number
  diameter: number
  netName?: string
}

export interface SchematicPowerSymbol {
  id: string
  type: 'vcc' | 'vdd' | 'vss' | 'gnd' | 'custom'
  voltage?: number
  x: number
  y: number
  rotation: number
  netName: string
}

export interface SchematicAnnotation {
  id: string
  type: 'text' | 'line' | 'rect' | 'circle' | 'arrow'
  x: number
  y: number
  width?: number
  height?: number
  text?: string
  fontSize?: number
  color: string
  strokeWidth?: number
  fill?: string
}

export interface SchematicConnection {
  componentId: string
  pinNumber: string
  netName: string
  x: number
  y: number
}

export interface SchematicValidationError {
  type: 'unconnected_pin' | 'duplicate_reference' | 'missing_value' | 'net_conflict'
  message: string
  objectId: string
  severity: 'error' | 'warning' | 'info'
}

export interface SchematicSimulation {
  id: string
  name: string
  type: 'dc' | 'ac' | 'transient' | 'noise' | 'distortion'
  parameters: SimulationParameter[]
  results?: SimulationResult[]
}

export interface SimulationParameter {
  name: string
  value: string | number
  unit?: string
}

export interface SimulationResult {
  signal: string
  data: Array<{
    x: number
    y: number
  }>
}

// Main Schematic type to match Rust backend
export interface Schematic {
  id: string
  name: string
  components: Component[]
  wires: Wire[]
  nets: Net[]
  labels: SchematicLabel[]
  metadata: SchematicMetadata
}

export interface SchematicMetadata {
  title?: string
  revision?: string
  date?: string
  sheetSize: string
  gridVisible: boolean
  gridSize: number
}

// Import types from project
export type { Component, Wire, Net } from './project'

// Re-export imports for internal use  
import type { Component, Wire, Net } from './project'

// Define additional types for useSmartWiring

export interface SchematicComponent extends Component {
  pins: ComponentPin[]
}

export interface ComponentPin {
  id: string
  number: string
  name: string
  x: number
  y: number
  type: 'input' | 'output' | 'bidirectional' | 'power' | 'ground' | 'passive'
}
