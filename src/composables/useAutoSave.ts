import { ref, watch, onMounted, onUnmounted } from 'vue'
import type { Ref, WatchSource } from 'vue'

export interface AutoSaveOptions {
  interval?: number // milliseconds
  key?: string // localStorage key
  onSave?: (data: any) => Promise<void> | void
  onRestore?: () => Promise<any> | any
  onError?: (error: Error) => void
  enabled?: boolean
  debounce?: number // milliseconds
  showNotification?: boolean
}

export interface AutoSaveState {
  lastSaved: Date | null
  isSaving: boolean
  error: Error | null
  isDirty: boolean
  saveCount: number
}

/**
 * Auto-save composable for automatic data persistence
 */
export function useAutoSave<T = any>(
  data: Ref<T> | WatchSource<T>,
  options: AutoSaveOptions = {}
) {
  const {
    interval = 30000, // 30 seconds default
    key = 'autosave',
    onSave,
    onRestore,
    onError,
    enabled = true,
    debounce = 1000,
    showNotification = false
  } = options

  // State
  const state = ref<AutoSaveState>({
    lastSaved: null,
    isSaving: false,
    error: null,
    isDirty: false,
    saveCount: 0
  })

  const isEnabled = ref(enabled)
  let saveTimer: NodeJS.Timeout | null = null
  let debounceTimer: NodeJS.Timeout | null = null
  let intervalTimer: NodeJS.Timeout | null = null

  /**
   * Save data
   */
  async function save(): Promise<void> {
    if (!isEnabled.value || state.value.isSaving) return

    state.value.isSaving = true
    state.value.error = null

    try {
      const dataValue = typeof data === 'function' ? data() : data.value

      if (onSave) {
        // Custom save handler
        await onSave(dataValue)
      } else {
        // Default: save to localStorage
        localStorage.setItem(key, JSON.stringify({
          data: dataValue,
          timestamp: Date.now(),
          version: '1.0.0'
        }))
      }

      state.value.lastSaved = new Date()
      state.value.isDirty = false
      state.value.saveCount++

      if (showNotification) {
        console.log('Auto-saved successfully')
      }
    } catch (error) {
      state.value.error = error as Error
      if (onError) {
        onError(error as Error)
      } else {
        console.error('Auto-save failed:', error)
      }
    } finally {
      state.value.isSaving = false
    }
  }

  /**
   * Restore data
   */
  async function restore(): Promise<T | null> {
    try {
      if (onRestore) {
        // Custom restore handler
        return await onRestore()
      } else {
        // Default: restore from localStorage
        const saved = localStorage.getItem(key)
        if (saved) {
          const parsed = JSON.parse(saved)
          state.value.lastSaved = new Date(parsed.timestamp)
          return parsed.data
        }
      }
    } catch (error) {
      state.value.error = error as Error
      if (onError) {
        onError(error as Error)
      } else {
        console.error('Auto-restore failed:', error)
      }
    }
    return null
  }

  /**
   * Mark as dirty and schedule save
   */
  function markDirty(): void {
    state.value.isDirty = true
    scheduleSave()
  }

  /**
   * Schedule a save operation
   */
  function scheduleSave(): void {
    if (!isEnabled.value) return

    // Clear existing timers
    if (debounceTimer) {
      clearTimeout(debounceTimer)
    }

    // Schedule debounced save
    debounceTimer = setTimeout(() => {
      save()
    }, debounce)
  }

  /**
   * Force immediate save
   */
  async function forceSave(): Promise<void> {
    // Clear any pending saves
    if (debounceTimer) {
      clearTimeout(debounceTimer)
      debounceTimer = null
    }
    if (saveTimer) {
      clearTimeout(saveTimer)
      saveTimer = null
    }

    await save()
  }

  /**
   * Clear saved data
   */
  function clear(): void {
    if (onSave) {
      console.warn('Cannot clear when using custom save handler')
      return
    }
    localStorage.removeItem(key)
    state.value.lastSaved = null
    state.value.isDirty = false
  }

  /**
   * Enable auto-save
   */
  function enable(): void {
    isEnabled.value = true
    startInterval()
  }

  /**
   * Disable auto-save
   */
  function disable(): void {
    isEnabled.value = false
    stopInterval()
  }

  /**
   * Start interval timer
   */
  function startInterval(): void {
    if (!isEnabled.value || intervalTimer) return

    intervalTimer = setInterval(() => {
      if (state.value.isDirty) {
        save()
      }
    }, interval)
  }

  /**
   * Stop interval timer
   */
  function stopInterval(): void {
    if (intervalTimer) {
      clearInterval(intervalTimer)
      intervalTimer = null
    }
  }

  /**
   * Get time since last save
   */
  function getTimeSinceLastSave(): number {
    if (!state.value.lastSaved) return Infinity
    return Date.now() - state.value.lastSaved.getTime()
  }

  /**
   * Format time since last save
   */
  function formatTimeSinceLastSave(): string {
    const ms = getTimeSinceLastSave()
    if (ms === Infinity) return 'Never'
    
    const seconds = Math.floor(ms / 1000)
    if (seconds < 60) return `${seconds} seconds ago`
    
    const minutes = Math.floor(seconds / 60)
    if (minutes < 60) return `${minutes} minute${minutes > 1 ? 's' : ''} ago`
    
    const hours = Math.floor(minutes / 60)
    if (hours < 24) return `${hours} hour${hours > 1 ? 's' : ''} ago`
    
    const days = Math.floor(hours / 24)
    return `${days} day${days > 1 ? 's' : ''} ago`
  }

  // Set up watchers
  if (isEnabled.value) {
    // Watch for data changes
    watch(
      data,
      () => {
        markDirty()
      },
      { deep: true }
    )
  }

  // Lifecycle hooks
  onMounted(() => {
    if (isEnabled.value) {
      startInterval()
    }

    // Save on page unload
    window.addEventListener('beforeunload', handleBeforeUnload)
  })

  onUnmounted(() => {
    // Clean up timers
    if (debounceTimer) clearTimeout(debounceTimer)
    if (saveTimer) clearTimeout(saveTimer)
    if (intervalTimer) clearInterval(intervalTimer)

    // Remove event listener
    window.removeEventListener('beforeunload', handleBeforeUnload)
  })

  // Handle page unload
  function handleBeforeUnload(e: BeforeUnloadEvent): void {
    if (state.value.isDirty && isEnabled.value) {
      // Try to save before leaving
      save()
      
      // Show confirmation dialog
      e.preventDefault()
      e.returnValue = 'You have unsaved changes. Are you sure you want to leave?'
    }
  }

  return {
    // State
    state,
    isEnabled,
    
    // Methods
    save,
    restore,
    forceSave,
    clear,
    enable,
    disable,
    markDirty,
    
    // Utilities
    getTimeSinceLastSave,
    formatTimeSinceLastSave
  }
}

/**
 * Auto-save manager for managing multiple auto-save instances
 */
export class AutoSaveManager {
  private instances: Map<string, ReturnType<typeof useAutoSave>> = new Map()

  /**
   * Register an auto-save instance
   */
  register(key: string, instance: ReturnType<typeof useAutoSave>): void {
    this.instances.set(key, instance)
  }

  /**
   * Unregister an auto-save instance
   */
  unregister(key: string): void {
    this.instances.delete(key)
  }

  /**
   * Save all instances
   */
  async saveAll(): Promise<void> {
    const promises = Array.from(this.instances.values()).map(instance => instance.save())
    await Promise.all(promises)
  }

  /**
   * Force save all instances
   */
  async forceSaveAll(): Promise<void> {
    const promises = Array.from(this.instances.values()).map(instance => instance.forceSave())
    await Promise.all(promises)
  }

  /**
   * Enable all instances
   */
  enableAll(): void {
    this.instances.forEach(instance => instance.enable())
  }

  /**
   * Disable all instances
   */
  disableAll(): void {
    this.instances.forEach(instance => instance.disable())
  }

  /**
   * Get instance by key
   */
  get(key: string): ReturnType<typeof useAutoSave> | undefined {
    return this.instances.get(key)
  }

  /**
   * Check if any instance is dirty
   */
  hasUnsavedChanges(): boolean {
    return Array.from(this.instances.values()).some(instance => instance.state.value.isDirty)
  }
}

// Global auto-save manager instance
export const autoSaveManager = new AutoSaveManager()