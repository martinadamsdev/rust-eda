import { invoke } from '@tauri-apps/api/core'

/**
 * Export API - Frontend interface to export functionality
 */
export const exportAPI = {
  /**
   * Export schematic to PDF
   */
  async exportToPDF(
    schematicId: string,
    outputPath: string
  ): Promise<void> {
    return await invoke<void>('export_to_pdf', {
      schematicId,
      outputPath
    })
  },

  /**
   * Export schematic to SVG
   */
  async exportToSVG(
    schematicId: string,
    outputPath: string
  ): Promise<string> {
    return await invoke<string>('export_to_svg', {
      schematicId,
      outputPath
    })
  },

  /**
   * Export netlist in specified format
   */
  async exportNetlist(
    schematicId: string,
    outputPath: string,
    format: 'spice' | 'verilog' | 'kicad'
  ): Promise<void> {
    return await invoke<void>('export_netlist', {
      schematicId,
      outputPath,
      format
    })
  },

  /**
   * Export Bill of Materials (BOM)
   */
  async exportBOM(
    outputPath: string,
    format: 'csv' | 'json' | 'html'
  ): Promise<void> {
    return await invoke<void>('export_bom', {
      outputPath,
      format
    })
  },

  /**
   * Export complete project archive
   */
  async exportProjectArchive(
    outputPath: string,
    includeLibraries: boolean = false
  ): Promise<void> {
    return await invoke<void>('export_project_archive', {
      outputPath,
      includeLibraries
    })
  }
}