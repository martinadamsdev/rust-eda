export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'

  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`
}

export function formatDate(date: Date | string): string {
  const d = typeof date === 'string' ? new Date(date) : date
  return d.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

export function formatComponentValue(value: string | number): string {
  if (typeof value === 'number') {
    return formatEngineeringNotation(value)
  }

  const numValue = parseComponentValue(value)
  if (numValue !== null) {
    return formatEngineeringNotation(numValue)
  }

  return value
}

export function parseComponentValue(value: string): number | null {
  const match = value.match(/^([\d.]+)\s*([kKmMuUnNpPfF])?/)
  if (!match) return null

  const num = parseFloat(match[1])
  const prefix = match[2]?.toLowerCase()

  const multipliers: Record<string, number> = {
    k: 1e3,
    m: 1e-3,
    u: 1e-6,
    n: 1e-9,
    p: 1e-12,
    f: 1e-15
  }

  return prefix ? num * multipliers[prefix] : num
}

export function formatEngineeringNotation(value: number): string {
  if (value === 0) return '0'
  
  const absValue = Math.abs(value)
  const sign = value < 0 ? '-' : ''
  
  const prefixes = [
    { value: 1e12, symbol: 'T' },
    { value: 1e9, symbol: 'G' },
    { value: 1e6, symbol: 'M' },
    { value: 1e3, symbol: 'k' },
    { value: 1, symbol: '' },
    { value: 1e-3, symbol: 'm' },
    { value: 1e-6, symbol: 'µ' },
    { value: 1e-9, symbol: 'n' },
    { value: 1e-12, symbol: 'p' },
    { value: 1e-15, symbol: 'f' }
  ]

  // Find the best prefix - the one that gives a value between 1 and 999
  let bestPrefix = prefixes[prefixes.length - 1] // default to smallest
  
  for (const prefix of prefixes) {
    const scaled = absValue / prefix.value
    if (scaled >= 1 && scaled < 1000) {
      bestPrefix = prefix
      break
    }
  }

  const scaled = absValue / bestPrefix.value
  
  // Format with appropriate precision
  let formatted: string
  if (scaled >= 100) {
    // For 100-999, show as integer if close to integer, otherwise 3 sig figs
    const rounded = Math.round(scaled)
    if (Math.abs(scaled - rounded) < 0.01) {
      formatted = rounded.toString()
    } else {
      formatted = scaled.toPrecision(3)
    }
  } else if (scaled >= 10) {
    // For 10-99, show 3 significant figures but drop trailing zeros
    formatted = parseFloat(scaled.toPrecision(3)).toString()
  } else if (scaled >= 1) {
    // For 1-9.99, show up to 3 significant figures
    formatted = parseFloat(scaled.toPrecision(3)).toString()
  } else {
    // For < 1, use exponential notation
    formatted = scaled.toExponential(2)
  }
  
  // Remove unnecessary decimal points
  formatted = formatted.replace(/\.0+$/, '')
  
  return `${sign}${formatted}${bestPrefix.symbol}`
}

export function formatCoordinate(value: number, unit: 'mm' | 'mil' | 'inch' = 'mm'): string {
  switch (unit) {
    case 'mm':
      return `${value.toFixed(2)} mm`
    case 'mil':
      return `${(value * 39.3701).toFixed(0)} mil`
    case 'inch':
      return `${(value / 25.4).toFixed(3)}"`
    default:
      return `${value.toFixed(2)}`
  }
}

export function formatAngle(degrees: number): string {
  const normalized = ((degrees % 360) + 360) % 360
  return `${normalized}°`
}

export function formatNetName(name: string): string {
  return name.toUpperCase().replace(/\s+/g, '_')
}

export function formatReference(prefix: string, number: number): string {
  return `${prefix.toUpperCase()}${number}`
}

export function truncateText(text: string, maxLength: number = 30): string {
  if (text.length <= maxLength) return text
  return `${text.substring(0, maxLength)}...`
}

export function formatPercentage(value: number, decimals: number = 0): string {
  return `${(value * 100).toFixed(decimals)}%`
}
