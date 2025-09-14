import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import type { Project, ProjectInfo } from '@/types'

export interface ProjectFile {
  name: string
  path: string
  content: string
}

export class TauriService {
  // Project operations
  static async createProject(name: string, projectPath: string): Promise<Project> {
    return invoke('create_project', { name, projectPath })
  }

  static async openProject(filePath: string): Promise<Project> {
    return invoke('open_project', { filePath })
  }

  static async openProjectDialog(): Promise<Project | null> {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'EDA Project',
          extensions: ['eda', 'json']
        }
      ]
    })

    if (selected) {
      return this.openProject(selected as string)
    }
    return null
  }

  static async saveProject(project: Project): Promise<void> {
    return invoke('save_project', { project })
  }

  static async getRecentProjects(): Promise<ProjectInfo[]> {
    return invoke('get_recent_projects')
  }

  // Greet example (from template)
  static async greet(name: string): Promise<string> {
    return invoke('greet', { name })
  }
}

export default TauriService
