/**
 * Central API export file
 * Provides unified access to all backend API modules
 */

export { projectAPI } from './project'
export { fileAPI } from './file'
export { schematicAPI } from './schematic'
export { exportAPI } from './export'

// Re-export as default object for convenience
import { projectAPI } from './project'
import { fileAPI } from './file'
import { schematicAPI } from './schematic'
import { exportAPI } from './export'

const api = {
  project: projectAPI,
  file: fileAPI,
  schematic: schematicAPI,
  export: exportAPI
}

export default api