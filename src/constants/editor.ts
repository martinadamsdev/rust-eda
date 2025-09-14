// Grid settings
export const GRID_SIZE = 10
export const GRID_COLOR = '#e0e0e0'
export const GRID_MAJOR_COLOR = '#c0c0c0'
export const GRID_MAJOR_INTERVAL = 5

// Canvas settings
export const DEFAULT_CANVAS_WIDTH = 800
export const DEFAULT_CANVAS_HEIGHT = 600
export const MIN_ZOOM = 0.1
export const MAX_ZOOM = 10
export const ZOOM_STEP = 0.1

// Component defaults
export const DEFAULT_COMPONENT_WIDTH = 80
export const DEFAULT_COMPONENT_HEIGHT = 60
export const COMPONENT_PADDING = 5
export const PIN_LENGTH = 20
export const PIN_RADIUS = 3

// Wire settings
export const WIRE_WIDTH = 2
export const WIRE_COLOR = '#000000'
export const WIRE_SELECTED_COLOR = '#2080f0'
export const WIRE_HOVER_COLOR = '#4090ff'
export const JUNCTION_RADIUS = 4

// Selection
export const SELECTION_COLOR = '#2080f0'
export const SELECTION_WIDTH = 2
export const SELECTION_PADDING = 5
export const HOVER_OPACITY = 0.5

// Performance
export const DIRTY_THRESHOLD = 10 // Number of dirty elements before full redraw
export const DEBOUNCE_TIME = 16 // ~60fps
export const MAX_UNDO_STACK = 100

// Component library
export const MAX_CACHE_SIZE = 100
export const CACHE_EXPIRY_TIME = 5 * 60 * 1000 // 5 minutes

// Auto-save
export const AUTO_SAVE_INTERVAL = 30000 // 30 seconds
export const AUTO_SAVE_DEBOUNCE = 1000 // 1 second

// Keyboard shortcuts
export const SHORTCUT_MODIFIER = navigator.platform.includes('Mac') ? 'meta' : 'ctrl'