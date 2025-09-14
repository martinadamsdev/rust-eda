import { invoke } from '@tauri-apps/api/core'
import { open, save } from '@tauri-apps/plugin-dialog'

/**
 * File API - Frontend interface to file system operations
 */
export const fileAPI = {
  /**
   * Show file open dialog
   */
  async showOpenDialog(): Promise<string | null> {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{
        name: 'EDA Project',
        extensions: ['eda', 'json']
      }]
    })
    return selected as string | null
  },

  /**
   * Show file save dialog
   */
  async showSaveDialog(defaultName?: string): Promise<string | null> {
    const selected = await save({
      defaultPath: defaultName,
      filters: [{
        name: 'EDA Project',
        extensions: ['eda', 'json']
      }]
    })
    return selected
  },

  /**
   * Read file contents as string
   */
  async readFile(path: string): Promise<string> {
    return await invoke<string>('read_file', { path })
  },

  /**
   * Write string content to file
   */
  async writeFile(path: string, content: string): Promise<void> {
    return await invoke<void>('write_file', { path, content })
  },

  /**
   * Check if file exists
   */
  async fileExists(path: string): Promise<boolean> {
    return await invoke<boolean>('file_exists', { path })
  },

  /**
   * Create directory (including parent directories)
   */
  async createDirectory(path: string): Promise<void> {
    return await invoke<void>('create_directory', { path })
  },

  /**
   * Get file information (size, modified date, etc.)
   */
  async getFileInfo(path: string): Promise<{
    size: number
    modified: string
    isDirectory: boolean
  }> {
    return await invoke('get_file_info', { path })
  }
}