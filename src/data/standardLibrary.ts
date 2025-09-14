import type { ComponentLibrary } from '@/types/library'
import {
  createResistor,
  createCapacitor,
  createInductor,
  createDiode,
  createLED,
  createTransistorNPN,
  createMOSFET_N,
  createOpAmp,
  createGround,
  createVCC,
  createConnector2Pin
} from './components'

/**
 * Get the standard component library
 * @returns ComponentLibrary with all standard components
 */
export function getStandardLibrary(): ComponentLibrary {
  return {
    id: 'standard',
    name: 'Standard Components',
    description: 'Basic electronic components library',
    version: '1.0.0',
    author: 'Rust EDA',
    license: 'MIT',
    components: [
      // Passive components
      createResistor(),
      createCapacitor(),
      createInductor(),
      
      // Active components  
      createDiode(),
      createLED(),
      createTransistorNPN(),
      createMOSFET_N(),
      
      // ICs
      createOpAmp(),
      
      // Power
      createGround(),
      createVCC(),
      
      // Connectors
      createConnector2Pin()
    ],
    config: {
      gridSize: 10,
      units: 'mm',
      defaultLineWidth: 2,
      defaultTextSize: 12
    },
    metadata: {
      createdAt: new Date(),
      updatedAt: new Date(),
      tags: ['basic', 'standard', 'electronic']
    }
  }
}