import { invoke } from '@tauri-apps/api/core'
import type { Project, Schematic } from '@/types'
import type { ProjectSettings } from '@/types/settings'

/**
 * Project API - Frontend interface to Rust backend project commands
 */
export const projectAPI = {
  /**
   * Create a new project
   */
  async createProject(name: string): Promise<Project> {
    return await invoke<Project>('create_project', { name })
  },

  /**
   * Open an existing project from file
   */
  async openProject(path: string): Promise<Project> {
    return await invoke<Project>('open_project', { path })
  },

  /**
   * Save the current project
   */
  async saveProject(project: Project, path: string): Promise<void> {
    return await invoke<void>('save_project', { project, path })
  },

  /**
   * Save project with a new path
   */
  async saveProjectAs(project: Project, newPath: string): Promise<void> {
    return await invoke<void>('save_project_as', { 
      project, 
      newPath 
    })
  },

  /**
   * Get list of recently opened projects
   */
  async getRecentProjects(): Promise<string[]> {
    return await invoke<string[]>('get_recent_projects')
  },

  /**
   * Add a new schematic to the project
   */
  async addSchematicToProject(
    projectId: string, 
    schematicName: string
  ): Promise<Schematic> {
    return await invoke<Schematic>('add_schematic_to_project', { 
      projectId, 
      schematicName 
    })
  },

  /**
   * Get project information by ID
   */
  async getProjectInfo(projectId: string): Promise<Project> {
    return await invoke<Project>('get_project_info', { projectId })
  },

  /**
   * Update project settings
   */
  async updateProjectSettings(
    projectId: string, 
    settings: ProjectSettings
  ): Promise<void> {
    return await invoke<void>('update_project_settings', { 
      projectId, 
      settings 
    })
  }
}