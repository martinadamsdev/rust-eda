import type { ComponentSymbol, ComponentCategory } from '@/types/library'
import { PinType } from '@/types/library'

export function createDiode(): ComponentSymbol {
  return {
    id: 'std_diode',
    name: 'Diode',
    category: 'active' as ComponentCategory,
    subcategory: 'Diodes',
    description: 'Standard diode',
    keywords: ['diode', 'd', 'rectifier', 'active'],
    graphics: {
      symbol: [
        { type: 'line', x1: -30, y1: 0, x2: -10, y2: 0, stroke: '#000', strokeWidth: 2 },
        { type: 'polygon', points: [
          { x: -10, y: -10 },
          { x: -10, y: 10 },
          { x: 10, y: 0 }
        ], fill: 'none', stroke: '#000' },
        { type: 'line', x1: 10, y1: -10, x2: 10, y2: 10, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: 10, y1: 0, x2: 30, y2: 0, stroke: '#000', strokeWidth: 2 }
      ],
      bounds: { x: -30, y: -10, width: 60, height: 20 }
    },
    pins: [
      { id: 'anode', name: 'A', number: '1', type: PinType.Passive, x: -30, y: 0 },
      { id: 'cathode', name: 'K', number: '2', type: PinType.Passive, x: 30, y: 0 }
    ],
    defaultProperties: {
      model: '1N4148',
      vf: '0.7V',
      current: '200mA'
    },
    parameters: [
      {
        name: 'model',
        label: 'Model',
        type: 'string',
        default: '1N4148'
      },
      {
        name: 'vf',
        label: 'Forward Voltage',
        type: 'string',
        default: '0.7V',
        unit: 'V'
      },
      {
        name: 'current',
        label: 'Current Rating',
        type: 'string',
        default: '200mA',
        unit: 'A'
      }
    ],
    footprints: [
      { name: 'D_SOD-123', description: 'SOD-123 SMD', recommended: true },
      { name: 'D_SOD-323', description: 'SOD-323 SMD' },
      { name: 'D_DO-35', description: 'DO-35 Through-hole' }
    ]
  }
}

export function createLED(): ComponentSymbol {
  return {
    id: 'std_led',
    name: 'LED',
    category: 'active' as ComponentCategory,
    subcategory: 'Diodes',
    description: 'Light Emitting Diode',
    keywords: ['led', 'light', 'diode', 'indicator', 'active'],
    graphics: {
      symbol: [
        { type: 'line', x1: -30, y1: 0, x2: -10, y2: 0, stroke: '#000', strokeWidth: 2 },
        { type: 'polygon', points: [
          { x: -10, y: -10 },
          { x: -10, y: 10 },
          { x: 10, y: 0 }
        ], fill: 'none', stroke: '#000' },
        { type: 'line', x1: 10, y1: -10, x2: 10, y2: 10, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: 10, y1: 0, x2: 30, y2: 0, stroke: '#000', strokeWidth: 2 },
        // Light arrows
        { type: 'line', x1: 0, y1: -15, x2: 8, y2: -23, stroke: '#000', strokeWidth: 1 },
        { type: 'polygon', points: [
          { x: 6, y: -22 },
          { x: 8, y: -23 },
          { x: 9, y: -21 }
        ], fill: '#000', stroke: '#000' },
        { type: 'line', x1: -5, y1: -15, x2: 3, y2: -23, stroke: '#000', strokeWidth: 1 },
        { type: 'polygon', points: [
          { x: 1, y: -22 },
          { x: 3, y: -23 },
          { x: 4, y: -21 }
        ], fill: '#000', stroke: '#000' }
      ],
      bounds: { x: -30, y: -23, width: 60, height: 33 }
    },
    pins: [
      { id: 'anode', name: 'A', number: '1', type: PinType.Passive, x: -30, y: 0 },
      { id: 'cathode', name: 'K', number: '2', type: PinType.Passive, x: 30, y: 0 }
    ],
    defaultProperties: {
      color: 'Red',
      vf: '2.0V',
      current: '20mA'
    },
    parameters: [
      {
        name: 'color',
        label: 'Color',
        type: 'select',
        default: 'Red',
        options: [
          { label: 'Red', value: 'Red' },
          { label: 'Green', value: 'Green' },
          { label: 'Blue', value: 'Blue' },
          { label: 'Yellow', value: 'Yellow' },
          { label: 'White', value: 'White' }
        ]
      },
      {
        name: 'vf',
        label: 'Forward Voltage',
        type: 'string',
        default: '2.0V',
        unit: 'V'
      },
      {
        name: 'current',
        label: 'Current',
        type: 'string',
        default: '20mA',
        unit: 'A'
      }
    ],
    footprints: [
      { name: 'LED_0603', description: '0603 SMD', recommended: true },
      { name: 'LED_0805', description: '0805 SMD' },
      { name: 'LED_THT_3mm', description: '3mm Through-hole' },
      { name: 'LED_THT_5mm', description: '5mm Through-hole' }
    ]
  }
}

export function createTransistorNPN(): ComponentSymbol {
  return {
    id: 'std_transistor_npn',
    name: 'NPN Transistor',
    category: 'active' as ComponentCategory,
    subcategory: 'Transistors',
    description: 'NPN Bipolar Junction Transistor',
    keywords: ['transistor', 'npn', 'bjt', 'active'],
    graphics: {
      symbol: [
        // Base line
        { type: 'line', x1: -30, y1: 0, x2: -10, y2: 0, stroke: '#000', strokeWidth: 2 },
        // Vertical line
        { type: 'line', x1: -10, y1: -15, x2: -10, y2: 15, stroke: '#000', strokeWidth: 3 },
        // Collector line
        { type: 'line', x1: -10, y1: -8, x2: 10, y2: -20, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: 10, y1: -20, x2: 10, y2: -30, stroke: '#000', strokeWidth: 2 },
        // Emitter line with arrow
        { type: 'line', x1: -10, y1: 8, x2: 10, y2: 20, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: 10, y1: 20, x2: 10, y2: 30, stroke: '#000', strokeWidth: 2 },
        // Arrow
        { type: 'polygon', points: [
          { x: 5, y: 15 },
          { x: 10, y: 20 },
          { x: 7, y: 17 }
        ], fill: '#000', stroke: '#000' }
      ],
      bounds: { x: -30, y: -30, width: 40, height: 60 }
    },
    pins: [
      { id: 'base', name: 'B', number: '1', type: PinType.Input, x: -30, y: 0 },
      { id: 'collector', name: 'C', number: '2', type: PinType.Passive, x: 10, y: -30 },
      { id: 'emitter', name: 'E', number: '3', type: PinType.Output, x: 10, y: 30 }
    ],
    defaultProperties: {
      model: '2N3904',
      hfe: '100',
      vceo: '40V'
    },
    parameters: [
      {
        name: 'model',
        label: 'Model',
        type: 'string',
        default: '2N3904'
      },
      {
        name: 'hfe',
        label: 'Current Gain',
        type: 'string',
        default: '100'
      },
      {
        name: 'vceo',
        label: 'Vceo Max',
        type: 'string',
        default: '40V',
        unit: 'V'
      }
    ],
    footprints: [
      { name: 'SOT-23', description: 'SOT-23 SMD', recommended: true },
      { name: 'SOT-223', description: 'SOT-223 SMD' },
      { name: 'TO-92', description: 'TO-92 Through-hole' }
    ]
  }
}

export function createMOSFET_N(): ComponentSymbol {
  return {
    id: 'std_mosfet_n',
    name: 'N-Channel MOSFET',
    category: 'active' as ComponentCategory,
    subcategory: 'Transistors',
    description: 'N-Channel MOSFET',
    keywords: ['mosfet', 'nmos', 'fet', 'transistor', 'active'],
    graphics: {
      symbol: [
        // Gate
        { type: 'line', x1: -30, y1: 0, x2: -15, y2: 0, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: -15, y1: -15, x2: -15, y2: 15, stroke: '#000', strokeWidth: 2 },
        // Channel lines
        { type: 'line', x1: -10, y1: -10, x2: -10, y2: -5, stroke: '#000', strokeWidth: 3 },
        { type: 'line', x1: -10, y1: -2, x2: -10, y2: 2, stroke: '#000', strokeWidth: 3 },
        { type: 'line', x1: -10, y1: 5, x2: -10, y2: 10, stroke: '#000', strokeWidth: 3 },
        // Drain
        { type: 'line', x1: -10, y1: -10, x2: 10, y2: -10, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: 10, y1: -10, x2: 10, y2: -30, stroke: '#000', strokeWidth: 2 },
        // Source
        { type: 'line', x1: -10, y1: 10, x2: 10, y2: 10, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: 10, y1: 10, x2: 10, y2: 30, stroke: '#000', strokeWidth: 2 },
        // Body diode arrow
        { type: 'line', x1: -10, y1: 0, x2: 0, y2: 0, stroke: '#000', strokeWidth: 2 },
        { type: 'polygon', points: [
          { x: -3, y: -3 },
          { x: 0, y: 0 },
          { x: -3, y: 3 }
        ], fill: '#000', stroke: '#000' },
        { type: 'line', x1: 0, y1: 0, x2: 10, y2: 0, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: 10, y1: 0, x2: 10, y2: 10, stroke: '#000', strokeWidth: 1 }
      ],
      bounds: { x: -30, y: -30, width: 40, height: 60 }
    },
    pins: [
      { id: 'gate', name: 'G', number: '1', type: PinType.Input, x: -30, y: 0 },
      { id: 'drain', name: 'D', number: '2', type: PinType.Passive, x: 10, y: -30 },
      { id: 'source', name: 'S', number: '3', type: PinType.Passive, x: 10, y: 30 }
    ],
    defaultProperties: {
      model: 'BSS138',
      vds: '50V',
      id: '200mA',
      rds: '3.5Ω'
    },
    parameters: [
      {
        name: 'model',
        label: 'Model',
        type: 'string',
        default: 'BSS138'
      },
      {
        name: 'vds',
        label: 'Vds Max',
        type: 'string',
        default: '50V',
        unit: 'V'
      },
      {
        name: 'id',
        label: 'Id Max',
        type: 'string',
        default: '200mA',
        unit: 'A'
      },
      {
        name: 'rds',
        label: 'Rds(on)',
        type: 'string',
        default: '3.5Ω',
        unit: 'Ω'
      }
    ],
    footprints: [
      { name: 'SOT-23', description: 'SOT-23 SMD', recommended: true },
      { name: 'SOT-323', description: 'SOT-323 SMD' },
      { name: 'TO-220', description: 'TO-220 Through-hole' }
    ]
  }
}