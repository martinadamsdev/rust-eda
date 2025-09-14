import type { ComponentSymbol, ComponentCategory } from '@/types/library'
import { PinType } from '@/types/library'

export function createOpAmp(): ComponentSymbol {
  return {
    id: 'std_opamp',
    name: 'Op-Amp',
    category: 'ic' as ComponentCategory,
    subcategory: 'Amplifiers',
    description: 'Operational Amplifier',
    keywords: ['opamp', 'amplifier', 'op-amp', 'ic'],
    graphics: {
      symbol: [
        // Triangle body
        { type: 'polygon', points: [
          { x: -20, y: -20 },
          { x: -20, y: 20 },
          { x: 20, y: 0 }
        ], fill: 'none', stroke: '#000' },
        // Input pins
        { type: 'line', x1: -30, y1: -10, x2: -20, y2: -10, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: -30, y1: 10, x2: -20, y2: 10, stroke: '#000', strokeWidth: 2 },
        // Output pin
        { type: 'line', x1: 20, y1: 0, x2: 30, y2: 0, stroke: '#000', strokeWidth: 2 },
        // + and - symbols
        { type: 'text', x: -15, y: -10, text: '+', fontSize: 12, fill: '#000' },
        { type: 'text', x: -15, y: 10, text: '-', fontSize: 12, fill: '#000' }
      ],
      bounds: { x: -30, y: -20, width: 60, height: 40 }
    },
    pins: [
      { id: 'in_p', name: 'IN+', number: '3', type: PinType.Input, x: -30, y: -10 },
      { id: 'in_n', name: 'IN-', number: '2', type: PinType.Input, x: -30, y: 10 },
      { id: 'out', name: 'OUT', number: '1', type: PinType.Output, x: 30, y: 0 },
      { id: 'vcc', name: 'VCC', number: '7', type: PinType.Power, x: 0, y: -20 },
      { id: 'vee', name: 'VEE', number: '4', type: PinType.Power, x: 0, y: 20 }
    ],
    defaultProperties: {
      model: 'LM358',
      supply: '±15V',
      gbw: '1MHz'
    },
    parameters: [
      {
        name: 'model',
        label: 'Model',
        type: 'string',
        default: 'LM358'
      },
      {
        name: 'supply',
        label: 'Supply Voltage',
        type: 'string',
        default: '±15V',
        unit: 'V'
      },
      {
        name: 'gbw',
        label: 'Gain Bandwidth',
        type: 'string',
        default: '1MHz',
        unit: 'Hz'
      }
    ],
    footprints: [
      { name: 'SOIC-8', description: 'SOIC-8 SMD', recommended: true },
      { name: 'DIP-8', description: 'DIP-8 Through-hole' },
      { name: 'MSOP-8', description: 'MSOP-8 SMD' }
    ]
  }
}