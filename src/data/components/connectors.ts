import type { ComponentSymbol, ComponentCategory } from '@/types/library'
import { PinType } from '@/types/library'

export function createConnector2Pin(): ComponentSymbol {
  return {
    id: 'std_conn_2pin',
    name: '2-Pin Connector',
    category: 'connector' as ComponentCategory,
    subcategory: 'Connectors',
    description: '2-pin connector',
    keywords: ['connector', 'header', 'pin', 'terminal'],
    graphics: {
      symbol: [
        { type: 'rect', x: -15, y: -20, width: 30, height: 40, fill: 'none', stroke: '#000' },
        { type: 'line', x1: -30, y1: -10, x2: -15, y2: -10, stroke: '#000', strokeWidth: 2 },
        { type: 'line', x1: -30, y1: 10, x2: -15, y2: 10, stroke: '#000', strokeWidth: 2 },
        { type: 'circle', cx: -10, cy: -10, r: 2, fill: '#000', stroke: '#000' },
        { type: 'circle', cx: -10, cy: 10, r: 2, fill: '#000', stroke: '#000' },
        { type: 'text', x: 0, y: -10, text: '1', fontSize: 10, fill: '#000' },
        { type: 'text', x: 0, y: 10, text: '2', fontSize: 10, fill: '#000' }
      ],
      bounds: { x: -30, y: -20, width: 45, height: 40 }
    },
    pins: [
      { id: 'pin1', name: '1', number: '1', type: PinType.BiDirectional, x: -30, y: -10 },
      { id: 'pin2', name: '2', number: '2', type: PinType.BiDirectional, x: -30, y: 10 }
    ],
    defaultProperties: {
      pitch: '2.54mm'
    },
    parameters: [
      {
        name: 'pitch',
        label: 'Pin Pitch',
        type: 'select',
        default: '2.54mm',
        options: [
          { label: '2.54mm', value: '2.54mm' },
          { label: '2.00mm', value: '2.00mm' },
          { label: '1.27mm', value: '1.27mm' }
        ]
      }
    ],
    footprints: [
      { name: 'PinHeader_1x02_P2.54mm', description: '2.54mm pitch header', recommended: true },
      { name: 'JST_XH_B2B-XH-A', description: 'JST XH 2-pin' },
      { name: 'TerminalBlock_2P_5.08mm', description: 'Terminal block 5.08mm' }
    ]
  }
}