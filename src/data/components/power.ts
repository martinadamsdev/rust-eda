import type { ComponentSymbol, ComponentCategory } from '@/types/library'
import { PinType } from '@/types/library'

export function createGround(): ComponentSymbol {
  return {
    id: 'std_ground',
    name: 'Ground',
    category: 'power' as ComponentCategory,
    subcategory: 'Power',
    description: 'Ground connection',
    keywords: ['ground', 'gnd', 'power', '0v'],
    graphics: {
      symbol: [
        { type: 'line', x1: 0, y1: 0, x2: 0, y2: 10, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: -15, y1: 10, x2: 15, y2: 10, stroke: '#000', strokeWidth: 3 },
        { type: 'line', x1: -10, y1: 15, x2: 10, y2: 15, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: -5, y1: 20, x2: 5, y2: 20, stroke: '#000', strokeWidth: 1 }
      ],
      bounds: { x: -15, y: 0, width: 30, height: 20 }
    },
    pins: [
      { id: 'gnd', name: 'GND', number: '1', type: PinType.Ground, x: 0, y: 0 }
    ],
    defaultProperties: {},
    parameters: [],
    footprints: []
  }
}

export function createVCC(): ComponentSymbol {
  return {
    id: 'std_vcc',
    name: 'VCC',
    category: 'power' as ComponentCategory,
    subcategory: 'Power',
    description: 'Power supply connection',
    keywords: ['vcc', 'vdd', 'power', 'supply'],
    graphics: {
      symbol: [
        { type: 'line', x1: 0, y1: 0, x2: 0, y2: 15, stroke: '#000', strokeWidth: 2 },
        { type: 'circle', cx: 0, cy: -5, r: 5, fill: 'none', stroke: '#000' },
        { type: 'text', x: 0, y: -5, text: '+', fontSize: 12, fill: '#000' }
      ],
      bounds: { x: -5, y: -10, width: 10, height: 25 }
    },
    pins: [
      { id: 'vcc', name: 'VCC', number: '1', type: PinType.Power, x: 0, y: 15 }
    ],
    defaultProperties: {
      voltage: '5V'
    },
    parameters: [
      {
        name: 'voltage',
        label: 'Voltage',
        type: 'string',
        default: '5V',
        unit: 'V'
      }
    ],
    footprints: []
  }
}