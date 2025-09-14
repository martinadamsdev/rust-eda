// Strongly typed settings interfaces

export interface ProjectSettings {
  gridSize: number
  snapToGrid: boolean
  autoSave: boolean
  autoSaveInterval: number
  defaultUnits: 'mm' | 'mil' | 'inch'
  colorScheme: 'light' | 'dark' | 'auto'
}

export interface ComponentProperties {
  reference?: string
  value?: string
  footprint?: string
  description?: string
  manufacturer?: string
  partNumber?: string
  tolerance?: string
  voltage?: string
  power?: string
  package?: string
}

export interface WireProperties {
  width?: number
  layer?: string
  netClass?: string
  impedance?: number
}

export interface SchematicSettings {
  title?: string
  revision?: string
  date?: string
  sheetSize: 'A4' | 'A3' | 'A2' | 'A1' | 'A0' | 'Letter' | 'Tabloid'
  gridVisible: boolean
  gridSize: number
  defaultLineWidth: number
  defaultTextSize: number
  snapRadius: number
}

export interface ExportSettings {
  includeLayers: string[]
  resolution?: number
  paperSize?: string
  orientation?: 'portrait' | 'landscape'
  margins?: {
    top: number
    right: number
    bottom: number
    left: number
  }
}

// Type guards
export function isValidProjectSettings(obj: any): obj is ProjectSettings {
  return (
    typeof obj === 'object' &&
    typeof obj.gridSize === 'number' &&
    typeof obj.snapToGrid === 'boolean' &&
    typeof obj.autoSave === 'boolean' &&
    typeof obj.autoSaveInterval === 'number' &&
    ['mm', 'mil', 'inch'].includes(obj.defaultUnits) &&
    ['light', 'dark', 'auto'].includes(obj.colorScheme)
  )
}

export function isValidComponentProperties(obj: any): obj is ComponentProperties {
  if (typeof obj !== 'object') return false
  
  // All properties are optional, but if they exist they must be strings
  const stringFields = ['reference', 'value', 'footprint', 'description', 
                       'manufacturer', 'partNumber', 'tolerance', 'voltage', 'power', 'package']
  
  for (const field of stringFields) {
    if (obj[field] !== undefined && typeof obj[field] !== 'string') {
      return false
    }
  }
  
  return true
}