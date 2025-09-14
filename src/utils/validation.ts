/**
 * Validation utilities for input sanitization and error prevention
 */

// Component validation
export function validateComponentId(id: string): boolean {
  if (!id || typeof id !== 'string') return false
  // Must be alphanumeric with underscores/hyphens, 1-100 chars
  return /^[a-zA-Z0-9_-]{1,100}$/.test(id)
}

export function validateComponentName(name: string): boolean {
  if (!name || typeof name !== 'string') return false
  // Allow letters, numbers, spaces, common symbols, 1-50 chars
  return name.length > 0 && name.length <= 50 && /^[\w\s\-_()]+$/.test(name)
}

export function validateComponentValue(value: string | number): boolean {
  if (value === null || value === undefined) return false
  
  if (typeof value === 'number') {
    return isFinite(value) && value >= 0
  }
  
  if (typeof value === 'string') {
    // Allow engineering notation (e.g., "10k", "100n", "4.7µF")
    return /^[\d.]+\s*[kKmMuUnNpPfFGgTt]?[ΩΩFfHhVvAaWw]?$/.test(value)
  }
  
  return false
}

// Coordinate validation
export function validateCoordinate(value: number): boolean {
  return typeof value === 'number' && 
         isFinite(value) && 
         value >= -10000 && 
         value <= 10000
}

export function validatePoint(point: { x: number; y: number }): boolean {
  return point && 
         typeof point === 'object' &&
         validateCoordinate(point.x) && 
         validateCoordinate(point.y)
}

export function validateBounds(bounds: { x: number; y: number; width: number; height: number }): boolean {
  return bounds &&
         typeof bounds === 'object' &&
         validateCoordinate(bounds.x) &&
         validateCoordinate(bounds.y) &&
         bounds.width > 0 && bounds.width <= 1000 &&
         bounds.height > 0 && bounds.height <= 1000
}

// Wire validation
export function validateWirePoints(points: Array<{ x: number; y: number }>): boolean {
  if (!Array.isArray(points) || points.length < 2) return false
  return points.every(validatePoint)
}

// File validation
export function validateFileName(name: string): boolean {
  if (!name || typeof name !== 'string') return false
  // Prevent directory traversal and invalid characters
  const invalidChars = /[<>:"|?*\x00-\x1f]/
  const traversalPattern = /\.\./
  
  return !invalidChars.test(name) && 
         !traversalPattern.test(name) &&
         name.length > 0 &&
         name.length <= 255
}

export function validateFileExtension(filename: string, allowedExtensions: string[]): boolean {
  const extension = filename.split('.').pop()?.toLowerCase()
  return extension ? allowedExtensions.includes(extension) : false
}

export function validateFileSize(size: number, maxSizeMB: number = 10): boolean {
  return size > 0 && size <= maxSizeMB * 1024 * 1024
}

// JSON validation
export function validateJSON(jsonString: string): boolean {
  try {
    JSON.parse(jsonString)
    return true
  } catch {
    return false
  }
}

// Project validation
export function validateProjectName(name: string): boolean {
  if (!name || typeof name !== 'string') return false
  return name.length >= 1 && name.length <= 100 && /^[\w\s\-_]+$/.test(name)
}

export function validateProjectVersion(version: string): boolean {
  // Semantic versioning pattern
  return /^\d+\.\d+\.\d+(-[\w.]+)?(\+[\w.]+)?$/.test(version)
}

// Library validation
export function validateLibraryImport(data: any): boolean {
  if (!data || typeof data !== 'object') return false
  
  // Check required fields
  if (!data.version || !validateProjectVersion(data.version)) return false
  if (!Array.isArray(data.components)) return false
  
  // Validate each component has required fields
  return data.components.every((comp: any) => 
    comp.id && 
    comp.name && 
    comp.category &&
    comp.graphics &&
    Array.isArray(comp.pins)
  )
}

// Sanitization functions
export function sanitizeHtml(input: string): string {
  const div = document.createElement('div')
  div.textContent = input
  return div.innerHTML
}

export function sanitizeFilePath(path: string): string {
  // Remove directory traversal attempts and invalid characters
  return path
    .replace(/\.\./g, '')
    .replace(/[<>:"|?*\x00-\x1f]/g, '')
    .replace(/^[/\\]+/, '') // Remove leading slashes
    .trim()
}

export function sanitizeComponentName(name: string): string {
  return name
    .replace(/[^\w\s\-_()]/g, '')
    .trim()
    .substring(0, 50)
}

// Error messages
export const ValidationErrors = {
  INVALID_COMPONENT_ID: 'Component ID must be alphanumeric with hyphens/underscores',
  INVALID_COMPONENT_NAME: 'Component name contains invalid characters',
  INVALID_COMPONENT_VALUE: 'Invalid component value format',
  INVALID_COORDINATES: 'Coordinates must be within valid range (-10000 to 10000)',
  INVALID_WIRE_POINTS: 'Wire must have at least 2 valid points',
  INVALID_FILE_NAME: 'File name contains invalid characters',
  INVALID_FILE_EXTENSION: 'File type not supported',
  INVALID_FILE_SIZE: 'File size exceeds maximum limit',
  INVALID_JSON: 'Invalid JSON format',
  INVALID_PROJECT_NAME: 'Project name contains invalid characters',
  INVALID_PROJECT_VERSION: 'Invalid version format (use semantic versioning)',
  INVALID_LIBRARY_DATA: 'Invalid library data format'
} as const

// Type guards
export function isValidComponent(obj: any): boolean {
  return obj &&
         typeof obj === 'object' &&
         validateComponentId(obj.id) &&
         validateComponentName(obj.name) &&
         obj.x !== undefined &&
         obj.y !== undefined
}

export function isValidWire(obj: any): boolean {
  return obj &&
         typeof obj === 'object' &&
         obj.id &&
         Array.isArray(obj.points) &&
         validateWirePoints(obj.points)
}

// Batch validation
export interface ValidationResult {
  valid: boolean
  errors: string[]
}

export function validateComponentData(data: any): ValidationResult {
  const errors: string[] = []
  
  if (!validateComponentId(data.id)) {
    errors.push(ValidationErrors.INVALID_COMPONENT_ID)
  }
  
  if (!validateComponentName(data.name)) {
    errors.push(ValidationErrors.INVALID_COMPONENT_NAME)
  }
  
  if (data.value && !validateComponentValue(data.value)) {
    errors.push(ValidationErrors.INVALID_COMPONENT_VALUE)
  }
  
  if (!validateCoordinate(data.x) || !validateCoordinate(data.y)) {
    errors.push(ValidationErrors.INVALID_COORDINATES)
  }
  
  return {
    valid: errors.length === 0,
    errors
  }
}

export function validateProjectData(data: any): ValidationResult {
  const errors: string[] = []
  
  if (!validateProjectName(data.name)) {
    errors.push(ValidationErrors.INVALID_PROJECT_NAME)
  }
  
  if (!validateProjectVersion(data.version)) {
    errors.push(ValidationErrors.INVALID_PROJECT_VERSION)
  }
  
  // Validate components
  if (Array.isArray(data.components)) {
    data.components.forEach((comp: any, index: number) => {
      const result = validateComponentData(comp)
      if (!result.valid) {
        errors.push(`Component ${index}: ${result.errors.join(', ')}`)
      }
    })
  }
  
  // Validate wires
  if (Array.isArray(data.wires)) {
    data.wires.forEach((wire: any, index: number) => {
      if (!isValidWire(wire)) {
        errors.push(`Wire ${index}: ${ValidationErrors.INVALID_WIRE_POINTS}`)
      }
    })
  }
  
  return {
    valid: errors.length === 0,
    errors
  }
}