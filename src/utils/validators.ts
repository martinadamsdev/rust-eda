import type { Project, Component, Wire, Net } from '@/types'

export function isValidProjectName(name: string): boolean {
  if (!name || name.trim().length === 0) return false
  if (name.length > 255) return false
  const invalidChars = /[<>:"/\\|?*]/
  return !invalidChars.test(name)
}

export function isValidReference(reference: string): boolean {
  if (!reference || reference.trim().length === 0) return false
  const validPattern = /^[A-Z]+\d+$/
  return validPattern.test(reference)
}

export function isValidNetName(name: string): boolean {
  if (!name || name.trim().length === 0) return false
  const validPattern = /^[A-Za-z_][A-Za-z0-9_]*$/
  return validPattern.test(name)
}

export function isValidComponentValue(value: string): boolean {
  if (!value || value.trim().length === 0) return false
  return value.length <= 100
}

export function validateProject(project: Partial<Project>): string[] {
  const errors: string[] = []

  if (!project.name || !isValidProjectName(project.name)) {
    errors.push('Invalid project name')
  }

  if (!project.path) {
    errors.push('Project path is required')
  }

  return errors
}

export function validateComponent(component: Partial<Component>): string[] {
  const errors: string[] = []

  if (!component.reference || !isValidReference(component.reference)) {
    errors.push('Invalid component reference (e.g., R1, C1, U1)')
  }

  if (!component.value || !isValidComponentValue(component.value)) {
    errors.push('Invalid component value')
  }

  if (component.x === undefined || component.y === undefined) {
    errors.push('Component position is required')
  }

  return errors
}

export function checkDuplicateReferences(components: Component[]): string[] {
  const references = new Map<string, number>()
  const duplicates: string[] = []

  for (const component of components) {
    const count = references.get(component.reference) || 0
    references.set(component.reference, count + 1)
  }

  references.forEach((count, ref) => {
    if (count > 1) {
      duplicates.push(ref)
    }
  })

  return duplicates
}

export function checkUnconnectedPins(
  components: Component[],
  wires: Wire[]
): Array<{ componentId: string; pinNumber: string }> {
  const unconnected: Array<{ componentId: string; pinNumber: string }> = []

  for (const component of components) {
    for (const pin of component.pins || []) {
      const isConnected = wires.some(wire => {
        const pinX = component.x + pin.x
        const pinY = component.y + pin.y
        return (
          (wire.startX === pinX && wire.startY === pinY) ||
          (wire.endX === pinX && wire.endY === pinY)
        )
      })

      if (!isConnected && pin.type !== 'no_connect') {
        unconnected.push({
          componentId: component.id,
          pinNumber: pin.number
        })
      }
    }
  }

  return unconnected
}

export function validateNetConnectivity(nets: Net[], wires: Wire[]): string[] {
  const errors: string[] = []

  for (const net of nets) {
    if (net.wires.length === 0) {
      errors.push(`Net "${net.name}" has no wires`)
    }

    const wireIds = new Set(wires.map(w => w.id))
    for (const wireId of net.wires) {
      if (!wireIds.has(wireId)) {
        errors.push(`Net "${net.name}" references non-existent wire ${wireId}`)
      }
    }
  }

  return errors
}
