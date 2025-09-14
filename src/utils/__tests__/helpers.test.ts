import { describe, it, expect } from 'vitest'
import { generateId, snapToGrid, clamp, debounce, throttle } from '../helpers'

describe('helpers', () => {
  describe('generateId', () => {
    it('should generate unique ids', () => {
      const id1 = generateId()
      const id2 = generateId()
      expect(id1).not.toBe(id2)
      // ID format: timestamp-randomstring
      expect(id1).toMatch(/^\d{13}-[a-z0-9]{9}$/)
    })
  })

  describe('snapToGrid', () => {
    it('should snap values to grid', () => {
      expect(snapToGrid(12, 10)).toBe(10)
      expect(snapToGrid(18, 10)).toBe(20)
      expect(snapToGrid(15, 10)).toBe(20)
      expect(snapToGrid(14, 10)).toBe(10)
      expect(snapToGrid(25, 5)).toBe(25)
    })
  })

  describe('clamp', () => {
    it('should clamp values within range', () => {
      expect(clamp(5, 0, 10)).toBe(5)
      expect(clamp(-5, 0, 10)).toBe(0)
      expect(clamp(15, 0, 10)).toBe(10)
    })
  })

  describe('debounce', () => {
    it('should debounce function calls', async () => {
      let count = 0
      const fn = () => count++
      const debounced = debounce(fn, 10)
      
      debounced()
      debounced()
      debounced()
      
      expect(count).toBe(0)
      
      await new Promise(resolve => setTimeout(resolve, 20))
      expect(count).toBe(1)
    })
  })

  describe('throttle', () => {
    it('should throttle function calls', async () => {
      let count = 0
      const fn = () => count++
      const throttled = throttle(fn, 10)
      
      throttled()
      throttled()
      throttled()
      
      expect(count).toBe(1)
      
      await new Promise(resolve => setTimeout(resolve, 20))
      
      throttled()
      expect(count).toBe(2)
    })
  })
})