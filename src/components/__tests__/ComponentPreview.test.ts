import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { nextTick } from 'vue'
import { createPinia, setActivePinia } from 'pinia'
import ComponentPreview from '../library/ComponentPreview.vue'
import type { ComponentTemplate } from '@/types/library'
import { PinType, ComponentCategory } from '@/types/library'

// Mock component
const mockComponent: ComponentTemplate = {
  id: 'test-resistor',
  name: 'Resistor',
  categoryId: 'passive',
  subcategory: 'Resistors',
  description: 'Standard resistor',
  keywords: ['resistor', 'passive'],
  symbol: {
    id: 'resistor-symbol',
    name: 'Resistor Symbol',
    width: 60,
    height: 20,
    draw_commands: [],
    graphics: {
      bounds: { width: 60, height: 20 }
    }
  },
  pins: [
    { id: 'pin1', name: '1', number: '1', x: -30, y: 0, pin_type: PinType.Passive, electrical: { voltage: null, current: null, impedance: null } },
    { id: 'pin2', name: '2', number: '2', x: 30, y: 0, pin_type: PinType.Passive, electrical: { voltage: null, current: null, impedance: null } }
  ],
  defaultProperties: {
    value: '10k',
    tolerance: '5%',
    power: '0.25W'
  }
}

// Mock canvas context
const mockContext = {
  clearRect: vi.fn(),
  save: vi.fn(),
  restore: vi.fn(),
  translate: vi.fn(),
  scale: vi.fn(),
  beginPath: vi.fn(),
  moveTo: vi.fn(),
  lineTo: vi.fn(),
  rect: vi.fn(),
  arc: vi.fn(),
  stroke: vi.fn(),
  fill: vi.fn(),
  fillRect: vi.fn(),
  strokeRect: vi.fn(),
  strokeStyle: '',
  fillStyle: '',
  lineWidth: 1,
  font: '',
  textAlign: '',
  textBaseline: '',
  fillText: vi.fn()
}

describe('ComponentPreview', () => {
  beforeEach(() => {
    // Set up Pinia
    setActivePinia(createPinia())
    
    // Mock canvas.getContext
    HTMLCanvasElement.prototype.getContext = vi.fn(() => mockContext as any)
    vi.clearAllMocks()
  })

  it('should render component information', () => {
    const wrapper = mount(ComponentPreview, {
      props: {
        component: mockComponent as any,
        isSelected: false,
        showFavorite: true
      }
    })

    expect(wrapper.find('.component-name').text()).toBe('Resistor')
    expect(wrapper.find('.component-category').text()).toBe('Resistors')
  })

  it('should apply selected class when selected', () => {
    const wrapper = mount(ComponentPreview, {
      props: {
        component: mockComponent as any,
        isSelected: true,
        showFavorite: false
      }
    })

    expect(wrapper.find('.component-preview').classes()).toContain('selected')
  })

  it('should emit click event', async () => {
    const wrapper = mount(ComponentPreview, {
      props: {
        component: mockComponent as any,
        isSelected: false,
        showFavorite: false
      }
    })

    await wrapper.find('.component-preview').trigger('click')
    
    expect(wrapper.emitted('click')).toBeTruthy()
    expect(wrapper.emitted('click')![0]).toEqual([mockComponent])
  })

  it('should render component graphics on canvas', async () => {
    mount(ComponentPreview, {
      props: {
        component: mockComponent as any,
        isSelected: false,
        showFavorite: false
      }
    })

    await nextTick()

    // Check if canvas drawing methods were called
    expect(mockContext.save).toHaveBeenCalled()
    expect(mockContext.translate).toHaveBeenCalled()
    expect(mockContext.beginPath).toHaveBeenCalled()
    expect(mockContext.stroke).toHaveBeenCalled()
    expect(mockContext.restore).toHaveBeenCalled()
  })

  it('should handle drag start', async () => {
    const wrapper = mount(ComponentPreview, {
      props: {
        component: mockComponent as any,
        isSelected: false,
        showFavorite: false
      }
    })

    const mockDataTransfer = {
      effectAllowed: '',
      setData: vi.fn(),
      setDragImage: vi.fn()
    }

    const dragEvent = new DragEvent('dragstart', {
      dataTransfer: mockDataTransfer as any
    })

    const container = wrapper.find('.preview-canvas').element as HTMLElement
    container.dispatchEvent(dragEvent)

    await nextTick()

    expect(mockDataTransfer.setData).toHaveBeenCalledWith(
      'application/json',
      expect.stringContaining('library-component')
    )
  })

  it('should toggle favorite', async () => {
    const wrapper = mount(ComponentPreview, {
      props: {
        component: mockComponent as any,
        isSelected: false,
        showFavorite: true
      }
    })

    const favoriteIcon = wrapper.find('.favorite-icon')
    expect(favoriteIcon.exists()).toBe(true)

    await favoriteIcon.trigger('click')
    
    expect(wrapper.emitted('toggleFavorite')).toBeTruthy()
    expect(wrapper.emitted('toggleFavorite')![0]).toEqual([mockComponent])
  })

  it('should redraw on component change', async () => {
    const wrapper = mount(ComponentPreview, {
      props: {
        component: mockComponent as any,
        isSelected: false,
        showFavorite: false
      }
    })

    vi.clearAllMocks()

    // Change component
    const newComponent = { ...mockComponent, name: 'Capacitor' }
    await wrapper.setProps({ component: newComponent })

    await nextTick()

    // Should trigger redraw
    expect(mockContext.clearRect).toHaveBeenCalled()
    expect(mockContext.save).toHaveBeenCalled()
  })

  it('should clean up on unmount', async () => {
    const removeEventListenerSpy = vi.spyOn(window, 'removeEventListener')
    
    const wrapper = mount(ComponentPreview, {
      props: {
        component: mockComponent as any,
        isSelected: false,
        showFavorite: false
      }
    })

    wrapper.unmount()

    expect(removeEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function))
  })
})