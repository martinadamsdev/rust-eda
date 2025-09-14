import { invoke } from '@tauri-apps/api/core'
import type { Component, Wire, Point } from '@/types'
import type { ComponentProperties } from '@/types/settings'

/**
 * Schematic API - Frontend interface to schematic editing commands
 */
export const schematicAPI = {
  /**
   * Add a new component to the schematic
   */
  async addComponent(
    schematicId: string,
    componentType: string,
    x: number,
    y: number
  ): Promise<Component> {
    return await invoke<Component>('add_component', {
      schematicId,
      componentType,
      x,
      y
    })
  },

  /**
   * Update component properties
   */
  async updateComponent(
    schematicId: string,
    componentId: string,
    properties: ComponentProperties
  ): Promise<void> {
    return await invoke<void>('update_component', {
      schematicId,
      componentId,
      properties
    })
  },

  /**
   * Delete a component from the schematic
   */
  async deleteComponent(
    schematicId: string,
    componentId: string
  ): Promise<void> {
    return await invoke<void>('delete_component', {
      schematicId,
      componentId
    })
  },

  /**
   * Add a wire connection
   */
  async addWire(
    schematicId: string,
    points: Point[],
    netName?: string
  ): Promise<Wire> {
    return await invoke<Wire>('add_wire', {
      schematicId,
      points,
      netName
    })
  },

  /**
   * Delete a wire
   */
  async deleteWire(
    schematicId: string,
    wireId: string
  ): Promise<void> {
    return await invoke<void>('delete_wire', {
      schematicId,
      wireId
    })
  },

  /**
   * Move a component to new position
   */
  async moveComponent(
    schematicId: string,
    componentId: string,
    x: number,
    y: number
  ): Promise<void> {
    return await invoke<void>('move_component', {
      schematicId,
      componentId,
      x,
      y
    })
  },

  /**
   * Rotate a component
   */
  async rotateComponent(
    schematicId: string,
    componentId: string,
    rotation: number
  ): Promise<void> {
    return await invoke<void>('rotate_component', {
      schematicId,
      componentId,
      rotation
    })
  },

  /**
   * Generate netlist from schematic
   */
  async generateNetlist(
    schematicId: string,
    format: 'spice' | 'verilog' | 'kicad'
  ): Promise<string> {
    return await invoke<string>('generate_netlist', {
      schematicId,
      format
    })
  },

  /**
   * Validate schematic for errors
   */
  async validateSchematic(schematicId: string): Promise<{
    valid: boolean
    errors: string[]
    warnings: string[]
  }> {
    return await invoke('validate_schematic', { schematicId })
  }
}