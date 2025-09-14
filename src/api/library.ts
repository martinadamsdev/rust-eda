import { invoke } from '@tauri-apps/api/core'

export interface ComponentLibrary {
  id: string
  name: string
  version: string
  description?: string
  categories: ComponentCategory[]
  components: Record<string, ComponentTemplate>
}

export interface ComponentCategory {
  id: string
  name: string
  parentId?: string
  description?: string
  color?: string
}

export interface ComponentTemplate {
  id: string
  name: string
  categoryId: string
  description?: string
  symbol: ComponentSymbol
  footprint?: string
  parameters: Record<string, ParameterTemplate>
  defaultProperties: Record<string, ComponentProperty>
  pins: PinTemplate[]
  keywords: string[]
}

export interface ComponentSymbol {
  width: number
  height: number
  drawCommands: DrawCommand[]
}

export interface DrawCommand {
  commandType: 'rectangle' | 'circle' | 'line' | 'arc' | 'text' | 'polygon'
  parameters: number[]
  style?: DrawStyle
}

export interface DrawStyle {
  strokeWidth: number
  strokeColor: string
  fillColor?: string
}

export interface ParameterTemplate {
  name: string
  parameterType: ParameterType
  defaultValue: any
  minValue?: number
  maxValue?: number
  unit?: string
  description?: string
  required: boolean
}

export type ParameterType = 
  | { type: 'String' }
  | { type: 'Number' }
  | { type: 'Boolean' }
  | { type: 'Choice'; choices: string[] }

export interface PinTemplate {
  id: string
  name: string
  number: string
  x: number
  y: number
  pinType: PinType
  electrical: ElectricalType
}

export type PinType = 'Input' | 'Output' | 'Bidirectional' | 'Power' | 'Ground' | 'Passive' | 'NotConnected'

export interface ElectricalType {
  voltage?: number
  current?: number
  impedance?: number
}

export interface ComponentProperty {
  value: any
  visible: boolean
  editable: boolean
}

// API Functions
export async function getAllLibraries(): Promise<ComponentLibrary[]> {
  return await invoke('get_all_libraries')
}

export async function getLibrary(libraryId: string): Promise<ComponentLibrary | null> {
  return await invoke('get_library', { libraryId })
}

export async function getComponentTemplate(
  libraryId: string,
  componentId: string
): Promise<ComponentTemplate | null> {
  return await invoke('get_component_template', { libraryId, componentId })
}

export async function searchComponents(query: string): Promise<[string, ComponentTemplate][]> {
  return await invoke('search_components', { query })
}

export async function getComponentsByCategory(
  categoryId: string
): Promise<[string, ComponentTemplate][]> {
  return await invoke('get_components_by_category', { categoryId })
}

export async function getLibraryCategories(libraryId: string): Promise<ComponentCategory[]> {
  return await invoke('get_library_categories', { libraryId })
}

export async function getAllCategories(): Promise<ComponentCategory[]> {
  return await invoke('get_all_categories')
}