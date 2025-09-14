import type { ComponentSymbol, ComponentCategory } from '@/types/library'
import { PinType } from '@/types/library'

export function createResistor(): ComponentSymbol {
  return {
    id: 'std_resistor',
    name: 'Resistor',
    category: 'passive' as ComponentCategory,
    subcategory: 'Resistors',
    description: 'Standard resistor',
    keywords: ['resistor', 'r', 'resistance', 'passive'],
    graphics: {
      symbol: [
        { type: 'line', x1: -30, y1: 0, x2: -20, y2: 0, stroke: '#000', strokeWidth: 2 },
        { type: 'rect', x: -20, y: -8, width: 40, height: 16, fill: 'none', stroke: '#000' },
        { type: 'line', x1: 20, y1: 0, x2: 30, y2: 0, stroke: '#000', strokeWidth: 2 }
      ],
      bounds: { x: -30, y: -8, width: 60, height: 16 }
    },
    pins: [
      { id: 'pin1', name: '1', number: '1', type: PinType.Passive, x: -30, y: 0 },
      { id: 'pin2', name: '2', number: '2', type: PinType.Passive, x: 30, y: 0 }
    ],
    defaultProperties: {
      value: '10k',
      tolerance: '5%',
      power: '0.25W'
    },
    parameters: [
      {
        name: 'resistance',
        label: 'Resistance',
        type: 'string',
        default: '10k',
        unit: 'Ω'
      },
      {
        name: 'tolerance',
        label: 'Tolerance',
        type: 'select',
        default: '5%',
        options: [
          { label: '1%', value: '1%' },
          { label: '5%', value: '5%' },
          { label: '10%', value: '10%' }
        ]
      },
      {
        name: 'power',
        label: 'Power Rating',
        type: 'select',
        default: '0.25W',
        options: [
          { label: '0.125W', value: '0.125W' },
          { label: '0.25W', value: '0.25W' },
          { label: '0.5W', value: '0.5W' },
          { label: '1W', value: '1W' }
        ]
      }
    ],
    footprints: [
      { name: 'R_0402', description: '0402 SMD', recommended: true },
      { name: 'R_0603', description: '0603 SMD' },
      { name: 'R_0805', description: '0805 SMD' },
      { name: 'R_THT_10mm', description: 'Through-hole 10mm' }
    ]
  }
}

export function createCapacitor(): ComponentSymbol {
  return {
    id: 'std_capacitor',
    name: 'Capacitor',
    category: 'passive' as ComponentCategory,
    subcategory: 'Capacitors',
    description: 'Standard capacitor',
    keywords: ['capacitor', 'c', 'capacitance', 'passive'],
    graphics: {
      symbol: [
        { type: 'line', x1: -30, y1: 0, x2: -10, y2: 0, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: -10, y1: -15, x2: -10, y2: 15, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: 10, y1: -15, x2: 10, y2: 15, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: 10, y1: 0, x2: 30, y2: 0, stroke: '#000', strokeWidth: 2 }
      ],
      bounds: { x: -30, y: -15, width: 60, height: 30 }
    },
    pins: [
      { id: 'pin1', name: '1', number: '1', type: PinType.Passive, x: -30, y: 0 },
      { id: 'pin2', name: '2', number: '2', type: PinType.Passive, x: 30, y: 0 }
    ],
    defaultProperties: {
      value: '100nF',
      voltage: '50V',
      type: 'Ceramic'
    },
    parameters: [
      {
        name: 'capacitance',
        label: 'Capacitance',
        type: 'string',
        default: '100nF',
        unit: 'F'
      },
      {
        name: 'voltage',
        label: 'Voltage Rating',
        type: 'string',
        default: '50V',
        unit: 'V'
      },
      {
        name: 'type',
        label: 'Type',
        type: 'select',
        default: 'Ceramic',
        options: [
          { label: 'Ceramic', value: 'Ceramic' },
          { label: 'Electrolytic', value: 'Electrolytic' },
          { label: 'Tantalum', value: 'Tantalum' },
          { label: 'Film', value: 'Film' }
        ]
      }
    ],
    footprints: [
      { name: 'C_0402', description: '0402 SMD', recommended: true },
      { name: 'C_0603', description: '0603 SMD' },
      { name: 'C_0805', description: '0805 SMD' },
      { name: 'CP_THT_5mm', description: 'Through-hole 5mm' }
    ]
  }
}

export function createInductor(): ComponentSymbol {
  return {
    id: 'std_inductor',
    name: 'Inductor',
    category: 'passive' as ComponentCategory,
    subcategory: 'Inductors',
    description: 'Standard inductor',
    keywords: ['inductor', 'l', 'inductance', 'coil', 'passive'],
    graphics: {
      symbol: [
        { type: 'line', x1: -30, y1: 0, x2: -20, y2: 0, stroke: '#000', strokeWidth: 2 },
        { type: 'arc', cx: -15, cy: 0, r: 5, startAngle: 180, endAngle: 0, stroke: '#000' },
        { type: 'arc', cx: -5, cy: 0, r: 5, startAngle: 180, endAngle: 0, stroke: '#000' },
        { type: 'arc', cx: 5, cy: 0, r: 5, startAngle: 180, endAngle: 0, stroke: '#000' },
        { type: 'arc', cx: 15, cy: 0, r: 5, startAngle: 180, endAngle: 0, stroke: '#000' },
        { type: 'line', x1: 20, y1: 0, x2: 30, y2: 0, stroke: '#000', strokeWidth: 2 }
      ],
      bounds: { x: -30, y: -5, width: 60, height: 10 }
    },
    pins: [
      { id: 'pin1', name: '1', number: '1', type: PinType.Passive, x: -30, y: 0 },
      { id: 'pin2', name: '2', number: '2', type: PinType.Passive, x: 30, y: 0 }
    ],
    defaultProperties: {
      value: '10uH',
      current: '1A',
      dcr: '0.1Ω'
    },
    parameters: [
      {
        name: 'inductance',
        label: 'Inductance',
        type: 'string',
        default: '10uH',
        unit: 'H'
      },
      {
        name: 'current',
        label: 'Current Rating',
        type: 'string',
        default: '1A',
        unit: 'A'
      },
      {
        name: 'dcr',
        label: 'DC Resistance',
        type: 'string',
        default: '0.1Ω',
        unit: 'Ω'
      }
    ],
    footprints: [
      { name: 'L_0402', description: '0402 SMD' },
      { name: 'L_0603', description: '0603 SMD' },
      { name: 'L_0805', description: '0805 SMD', recommended: true },
      { name: 'L_THT_7mm', description: 'Through-hole 7mm' }
    ]
  }
}