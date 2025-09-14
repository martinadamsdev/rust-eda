import { shallowRef, ref, computed } from 'vue'

export interface HistoryEntry<T = any> {
  id: string
  timestamp: number
  description: string
  data: T
  type: 'create' | 'update' | 'delete' | 'batch'
}

export interface HistoryOptions {
  maxSize?: number
  debounceTime?: number
}

/**
 * Create a history manager for undo/redo functionality
 */
export function useHistory<T = any>(options: HistoryOptions = {}) {
  const maxSize = options.maxSize || 100
  const debounceTime = options.debounceTime || 0

  const history = shallowRef<HistoryEntry<T>[]>([])
  const currentIndex = ref(-1)
  const isRecording = ref(true)
  let debounceTimer: NodeJS.Timeout | null = null

  // Computed properties
  const canUndo = computed(() => currentIndex.value >= 0)
  const canRedo = computed(() => currentIndex.value < history.value.length - 1)
  const currentEntry = computed(() => history.value[currentIndex.value] || null)
  
  const undoStack = computed(() => history.value.slice(0, currentIndex.value + 1))
  const redoStack = computed(() => history.value.slice(currentIndex.value + 1))

  /**
   * Add entry to history
   */
  function push(entry: Omit<HistoryEntry<T>, 'id' | 'timestamp'>): void {
    if (!isRecording.value) return

    // Clear debounce timer if exists
    if (debounceTimer) {
      clearTimeout(debounceTimer)
      debounceTimer = null
    }

    const addEntry = () => {
      // Remove any entries after current index (clear redo stack)
      if (currentIndex.value < history.value.length - 1) {
        history.value = history.value.slice(0, currentIndex.value + 1)
      }

      // Add new entry
      const newEntry: HistoryEntry<T> = {
        ...entry,
        id: crypto.randomUUID(),
        timestamp: Date.now()
      }

      history.value.push(newEntry)
      currentIndex.value++

      // Enforce max size
      if (history.value.length > maxSize) {
        history.value.shift()
        currentIndex.value--
      }
    }

    if (debounceTime > 0) {
      debounceTimer = setTimeout(addEntry, debounceTime)
    } else {
      addEntry()
    }
  }

  /**
   * Undo last action
   */
  function undo(): HistoryEntry<T> | null {
    if (!canUndo.value) return null

    const entry = history.value[currentIndex.value]
    currentIndex.value--
    return entry
  }

  /**
   * Redo next action
   */
  function redo(): HistoryEntry<T> | null {
    if (!canRedo.value) return null

    currentIndex.value++
    return history.value[currentIndex.value]
  }

  /**
   * Clear all history
   */
  function clear(): void {
    history.value = []
    currentIndex.value = -1
  }

  /**
   * Go to specific history entry
   */
  function goto(index: number): HistoryEntry<T> | null {
    if (index < 0 || index >= history.value.length) return null
    
    currentIndex.value = index
    return history.value[index]
  }

  /**
   * Batch multiple operations into single history entry
   */
  function batch<R>(fn: () => R, description: string): R {
    const originalRecording = isRecording.value
    const batchData: T[] = []
    
    // Temporarily stop recording individual operations
    isRecording.value = false
    
    try {
      const result = fn()
      
      // Add batch entry if any data was collected
      if (batchData.length > 0) {
        isRecording.value = originalRecording
        push({
          type: 'batch',
          description,
          data: batchData as T
        })
      }
      
      return result
    } finally {
      isRecording.value = originalRecording
    }
  }

  /**
   * Start recording history
   */
  function startRecording(): void {
    isRecording.value = true
  }

  /**
   * Stop recording history
   */
  function stopRecording(): void {
    isRecording.value = false
  }

  /**
   * Get history summary
   */
  function getSummary(): string[] {
    return history.value.map((entry, index) => {
      const marker = index === currentIndex.value ? '→ ' : '  '
      return `${marker}${entry.description} (${new Date(entry.timestamp).toLocaleTimeString()})`
    })
  }

  return {
    // State
    history,
    currentIndex,
    isRecording,
    
    // Computed
    canUndo,
    canRedo,
    currentEntry,
    undoStack,
    redoStack,
    
    // Methods
    push,
    undo,
    redo,
    clear,
    goto,
    batch,
    startRecording,
    stopRecording,
    getSummary
  }
}

/**
 * Create a command pattern implementation for undo/redo
 */
export interface Command<T = any> {
  execute(): T
  undo(): void
  redo(): void
  description: string
}

export class CommandManager<T = any> {
  private history: Command<T>[] = []
  private currentIndex = -1
  private maxSize = 100

  constructor(maxSize = 100) {
    this.maxSize = maxSize
  }

  /**
   * Execute a command and add to history
   */
  execute(command: Command<T>): T {
    // Clear redo stack
    if (this.currentIndex < this.history.length - 1) {
      this.history = this.history.slice(0, this.currentIndex + 1)
    }

    // Execute command
    const result = command.execute()

    // Add to history
    this.history.push(command)
    this.currentIndex++

    // Enforce max size
    if (this.history.length > this.maxSize) {
      this.history.shift()
      this.currentIndex--
    }

    return result
  }

  /**
   * Undo last command
   */
  undo(): boolean {
    if (this.currentIndex < 0) return false

    const command = this.history[this.currentIndex]
    command.undo()
    this.currentIndex--
    return true
  }

  /**
   * Redo next command
   */
  redo(): boolean {
    if (this.currentIndex >= this.history.length - 1) return false

    this.currentIndex++
    const command = this.history[this.currentIndex]
    command.redo()
    return true
  }

  /**
   * Check if can undo
   */
  canUndo(): boolean {
    return this.currentIndex >= 0
  }

  /**
   * Check if can redo
   */
  canRedo(): boolean {
    return this.currentIndex < this.history.length - 1
  }

  /**
   * Clear history
   */
  clear(): void {
    this.history = []
    this.currentIndex = -1
  }

  /**
   * Get history size
   */
  size(): number {
    return this.history.length
  }

  /**
   * Get current position
   */
  position(): number {
    return this.currentIndex
  }
}