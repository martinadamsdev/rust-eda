import { describe, it, expect } from 'vitest'
import {
  formatFileSize,
  formatComponentValue,
  parseComponentValue,
  formatEngineeringNotation,
  formatCoordinate,
  formatAngle,
  formatNetName,
  formatReference,
  truncateText,
  formatPercentage
} from '../formatters'

describe('formatters', () => {
  describe('formatFileSize', () => {
    it('should format bytes correctly', () => {
      expect(formatFileSize(0)).toBe('0 B')
      expect(formatFileSize(1024)).toBe('1 KB')
      expect(formatFileSize(1024 * 1024)).toBe('1 MB')
      expect(formatFileSize(1536)).toBe('1.5 KB')
    })
  })

  describe('formatComponentValue', () => {
    it('should format component values correctly', () => {
      expect(formatComponentValue('10k')).toBe('10k')
      expect(formatComponentValue(0.001)).toBe('1m')
      expect(formatComponentValue(1000000)).toBe('1M')
    })
  })

  describe('parseComponentValue', () => {
    it('should parse component values correctly', () => {
      expect(parseComponentValue('10k')).toBe(10000)
      expect(parseComponentValue('100n')).toBeCloseTo(0.0000001, 10)
      expect(parseComponentValue('2.2M')).toBe(0.0022)
      expect(parseComponentValue('invalid')).toBe(null)
    })
  })

  describe('formatEngineeringNotation', () => {
    it('should format numbers in engineering notation', () => {
      expect(formatEngineeringNotation(1000)).toBe('1k')
      expect(formatEngineeringNotation(0.001)).toBe('1m')
      expect(formatEngineeringNotation(0.0000001)).toBe('100n')
      expect(formatEngineeringNotation(1234567)).toBe('1.23M')
    })
  })

  describe('formatCoordinate', () => {
    it('should format coordinates with units', () => {
      expect(formatCoordinate(10, 'mm')).toBe('10.00 mm')
      expect(formatCoordinate(10, 'mil')).toBe('394 mil')
      expect(formatCoordinate(25.4, 'inch')).toBe('1.000"')
    })
  })

  describe('formatAngle', () => {
    it('should normalize and format angles', () => {
      expect(formatAngle(90)).toBe('90°')
      expect(formatAngle(370)).toBe('10°')
      expect(formatAngle(-10)).toBe('350°')
    })
  })

  describe('formatNetName', () => {
    it('should format net names correctly', () => {
      expect(formatNetName('vcc in')).toBe('VCC_IN')
      expect(formatNetName('gnd  bus')).toBe('GND_BUS')
    })
  })

  describe('formatReference', () => {
    it('should format component references', () => {
      expect(formatReference('r', 1)).toBe('R1')
      expect(formatReference('c', 10)).toBe('C10')
      expect(formatReference('u', 3)).toBe('U3')
    })
  })

  describe('truncateText', () => {
    it('should truncate long text', () => {
      expect(truncateText('short text', 20)).toBe('short text')
      expect(truncateText('this is a very long text that needs truncation', 20)).toBe('this is a very long ...')
    })
  })

  describe('formatPercentage', () => {
    it('should format percentages correctly', () => {
      expect(formatPercentage(0.5)).toBe('50%')
      expect(formatPercentage(0.123, 1)).toBe('12.3%')
      expect(formatPercentage(1, 0)).toBe('100%')
    })
  })
})