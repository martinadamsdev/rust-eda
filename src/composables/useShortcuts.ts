import { onMounted, onUnmounted } from 'vue'
import type { Ref } from 'vue'

export interface ShortcutDefinition {
  key: string
  ctrl?: boolean
  shift?: boolean
  alt?: boolean
  meta?: boolean
  description: string
  action: () => void
  enabled?: Ref<boolean> | boolean
}

export interface ShortcutGroup {
  name: string
  shortcuts: ShortcutDefinition[]
}

/**
 * Global shortcut registry
 */
class ShortcutRegistry {
  private shortcuts: Map<string, ShortcutDefinition> = new Map()
  private groups: Map<string, ShortcutGroup> = new Map()
  private isListening = false

  /**
   * Generate unique key for shortcut
   */
  private generateKey(def: ShortcutDefinition): string {
    const parts = []
    if (def.ctrl) parts.push('ctrl')
    if (def.shift) parts.push('shift')
    if (def.alt) parts.push('alt')
    if (def.meta) parts.push('meta')
    parts.push(def.key.toLowerCase())
    return parts.join('+')
  }

  /**
   * Register a shortcut
   */
  register(shortcut: ShortcutDefinition): void {
    const key = this.generateKey(shortcut)
    this.shortcuts.set(key, shortcut)
  }

  /**
   * Unregister a shortcut
   */
  unregister(shortcut: ShortcutDefinition): void {
    const key = this.generateKey(shortcut)
    this.shortcuts.delete(key)
  }

  /**
   * Register a group of shortcuts
   */
  registerGroup(group: ShortcutGroup): void {
    this.groups.set(group.name, group)
    group.shortcuts.forEach(shortcut => this.register(shortcut))
  }

  /**
   * Unregister a group of shortcuts
   */
  unregisterGroup(groupName: string): void {
    const group = this.groups.get(groupName)
    if (group) {
      group.shortcuts.forEach(shortcut => this.unregister(shortcut))
      this.groups.delete(groupName)
    }
  }

  /**
   * Handle keyboard event
   */
  private handleKeyDown = (e: KeyboardEvent): void => {
    // Don't handle shortcuts when typing in input fields
    const target = e.target as HTMLElement
    if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.contentEditable === 'true') {
      return
    }

    const parts = []
    if (e.ctrlKey || e.metaKey) parts.push('ctrl')
    if (e.shiftKey) parts.push('shift')
    if (e.altKey) parts.push('alt')
    parts.push(e.key.toLowerCase())
    
    const key = parts.join('+')
    const shortcut = this.shortcuts.get(key)
    
    if (shortcut) {
      // Check if shortcut is enabled
      const enabled = typeof shortcut.enabled === 'object' 
        ? shortcut.enabled.value 
        : shortcut.enabled !== false
      
      if (enabled) {
        e.preventDefault()
        e.stopPropagation()
        shortcut.action()
      }
    }
  }

  /**
   * Start listening for keyboard events
   */
  startListening(): void {
    if (!this.isListening) {
      window.addEventListener('keydown', this.handleKeyDown)
      this.isListening = true
    }
  }

  /**
   * Stop listening for keyboard events
   */
  stopListening(): void {
    if (this.isListening) {
      window.removeEventListener('keydown', this.handleKeyDown)
      this.isListening = false
    }
  }

  /**
   * Get all registered shortcuts
   */
  getShortcuts(): ShortcutDefinition[] {
    return Array.from(this.shortcuts.values())
  }

  /**
   * Get shortcuts by group
   */
  getGroup(groupName: string): ShortcutGroup | undefined {
    return this.groups.get(groupName)
  }

  /**
   * Get all groups
   */
  getGroups(): ShortcutGroup[] {
    return Array.from(this.groups.values())
  }
}

// Singleton instance
const registry = new ShortcutRegistry()

/**
 * Composable for managing keyboard shortcuts
 */
export function useShortcuts() {
  onMounted(() => {
    registry.startListening()
  })

  onUnmounted(() => {
    // Don't stop listening as other components might still need it
  })

  return {
    register: (shortcut: ShortcutDefinition) => registry.register(shortcut),
    unregister: (shortcut: ShortcutDefinition) => registry.unregister(shortcut),
    registerGroup: (group: ShortcutGroup) => registry.registerGroup(group),
    unregisterGroup: (groupName: string) => registry.unregisterGroup(groupName),
    getShortcuts: () => registry.getShortcuts(),
    getGroups: () => registry.getGroups()
  }
}

/**
 * Predefined shortcut groups
 */
export const editorShortcuts: ShortcutGroup = {
  name: 'Editor',
  shortcuts: [
    {
      key: 's',
      ctrl: true,
      description: 'Save project',
      action: () => console.log('Save project')
    },
    {
      key: 'z',
      ctrl: true,
      description: 'Undo',
      action: () => console.log('Undo')
    },
    {
      key: 'y',
      ctrl: true,
      description: 'Redo',
      action: () => console.log('Redo')
    },
    {
      key: 'z',
      ctrl: true,
      shift: true,
      description: 'Redo',
      action: () => console.log('Redo')
    },
    {
      key: 'a',
      ctrl: true,
      description: 'Select all',
      action: () => console.log('Select all')
    },
    {
      key: 'c',
      ctrl: true,
      description: 'Copy',
      action: () => console.log('Copy')
    },
    {
      key: 'v',
      ctrl: true,
      description: 'Paste',
      action: () => console.log('Paste')
    },
    {
      key: 'x',
      ctrl: true,
      description: 'Cut',
      action: () => console.log('Cut')
    },
    {
      key: 'Delete',
      description: 'Delete selected',
      action: () => console.log('Delete')
    },
    {
      key: 'Escape',
      description: 'Cancel/Clear selection',
      action: () => console.log('Cancel')
    }
  ]
}

export const toolShortcuts: ShortcutGroup = {
  name: 'Tools',
  shortcuts: [
    {
      key: '1',
      description: 'Select tool',
      action: () => console.log('Select tool')
    },
    {
      key: '2',
      description: 'Wire tool',
      action: () => console.log('Wire tool')
    },
    {
      key: '3',
      description: 'Component tool',
      action: () => console.log('Component tool')
    },
    {
      key: '4',
      description: 'Move tool',
      action: () => console.log('Move tool')
    },
    {
      key: '5',
      description: 'Delete tool',
      action: () => console.log('Delete tool')
    },
    {
      key: 'r',
      description: 'Rotate component',
      action: () => console.log('Rotate')
    },
    {
      key: 'f',
      description: 'Flip component',
      action: () => console.log('Flip')
    }
  ]
}

export const viewShortcuts: ShortcutGroup = {
  name: 'View',
  shortcuts: [
    {
      key: '+',
      ctrl: true,
      description: 'Zoom in',
      action: () => console.log('Zoom in')
    },
    {
      key: '-',
      ctrl: true,
      description: 'Zoom out',
      action: () => console.log('Zoom out')
    },
    {
      key: '0',
      ctrl: true,
      description: 'Reset zoom',
      action: () => console.log('Reset zoom')
    },
    {
      key: 'g',
      description: 'Toggle grid',
      action: () => console.log('Toggle grid')
    },
    {
      key: 'l',
      ctrl: true,
      description: 'Toggle layers',
      action: () => console.log('Toggle layers')
    }
  ]
}