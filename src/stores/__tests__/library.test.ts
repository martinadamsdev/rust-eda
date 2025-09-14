import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useLibraryStore } from '../library'
import type { ComponentSymbol, ComponentLibrary } from '@/types/library'
import { ComponentCategory } from '@/types/library'

const mockComponent: ComponentSymbol = {
  id: 'test-component',
  name: 'Test Component',
  category: ComponentCategory.Passive,
  keywords: ['test'],
  graphics: {
    symbol: [],
    bounds: { x: 0, y: 0, width: 100, height: 100 }
  },
  pins: [],
  defaultProperties: {}
}

const mockLibrary: ComponentLibrary = {
  id: 'test-library',
  name: 'Test Library',
  version: '1.0.0',
  components: [mockComponent]
}

describe('Library Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  describe('Library Management', () => {
    it('should initialize with standard library', () => {
      const store = useLibraryStore()
      
      expect(store.libraries.length).toBeGreaterThan(0)
      expect(store.activeLibraryId).toBeTruthy()
      expect(store.activeLibrary).toBeTruthy()
    })

    it('should load a new library', () => {
      const store = useLibraryStore()
      const initialCount = store.libraries.length
      
      store.loadLibrary(mockLibrary)
      
      expect(store.libraries.length).toBe(initialCount + 1)
      expect(store.libraries.find(lib => lib.id === 'test-library')).toBeTruthy()
    })

    it('should update existing library when loading with same id', () => {
      const store = useLibraryStore()
      store.loadLibrary(mockLibrary)
      
      const updatedLibrary = { ...mockLibrary, name: 'Updated Library' }
      store.loadLibrary(updatedLibrary)
      
      const library = store.libraries.find(lib => lib.id === 'test-library')
      expect(library?.name).toBe('Updated Library')
    })

    it('should remove library', () => {
      const store = useLibraryStore()
      store.loadLibrary(mockLibrary)
      const initialCount = store.libraries.length
      
      store.removeLibrary('test-library')
      
      expect(store.libraries.length).toBe(initialCount - 1)
      expect(store.libraries.find(lib => lib.id === 'test-library')).toBeFalsy()
    })

    it('should set active library', () => {
      const store = useLibraryStore()
      store.loadLibrary(mockLibrary)
      
      store.setActiveLibrary('test-library')
      
      expect(store.activeLibraryId).toBe('test-library')
      expect(store.activeLibrary?.id).toBe('test-library')
    })
  })

  describe('Component Access', () => {
    it('should get component by ID', () => {
      const store = useLibraryStore()
      store.loadLibrary(mockLibrary)
      
      const component = store.getComponent('test-component')
      
      expect(component).toBeTruthy()
      expect(component?.id).toBe('test-component')
    })

    it('should return null for non-existent component', () => {
      const store = useLibraryStore()
      
      const component = store.getComponent('non-existent')
      
      expect(component).toBeNull()
    })

    it('should get all components from all libraries', () => {
      const store = useLibraryStore()
      const secondLibrary: ComponentLibrary = {
        id: 'second-library',
        name: 'Second Library',
        version: '1.0.0',
        components: [
          { ...mockComponent, id: 'second-component', name: 'Second Component' }
        ]
      }
      
      store.loadLibrary(mockLibrary)
      store.loadLibrary(secondLibrary)
      
      const allComponents = store.allComponents
      
      expect(allComponents.length).toBeGreaterThan(2)
      expect(allComponents.find(c => c.id === 'test-component')).toBeTruthy()
      expect(allComponents.find(c => c.id === 'second-component')).toBeTruthy()
    })
  })

  describe('Filtering', () => {
    it('should filter components by category', () => {
      const store = useLibraryStore()
      const testLibrary: ComponentLibrary = {
        id: 'test-lib',
        name: 'Test',
        version: '1.0.0',
        components: [
          { ...mockComponent, id: 'comp1', category: ComponentCategory.Passive },
          { ...mockComponent, id: 'comp2', category: ComponentCategory.Active },
          { ...mockComponent, id: 'comp3', category: ComponentCategory.Passive }
        ]
      }
      
      store.loadLibrary(testLibrary)
      store.setActiveLibrary('test-lib')
      store.selectedCategory = ComponentCategory.Passive
      
      const filtered = store.filteredComponents
      
      expect(filtered).toHaveLength(2)
      expect(filtered.every(c => c.category === ComponentCategory.Passive)).toBe(true)
    })

    it('should filter components by search query', () => {
      const store = useLibraryStore()
      const testLibrary: ComponentLibrary = {
        id: 'test-lib',
        name: 'Test',
        version: '1.0.0',
        components: [
          { ...mockComponent, id: 'r1', name: 'Resistor', keywords: ['passive'] },
          { ...mockComponent, id: 'c1', name: 'Capacitor', keywords: ['passive'] },
          { ...mockComponent, id: 'l1', name: 'Inductor', keywords: ['passive'] }
        ]
      }
      
      store.loadLibrary(testLibrary)
      store.setActiveLibrary('test-lib')
      store.searchQuery = 'cap'
      
      const filtered = store.filteredComponents
      
      expect(filtered).toHaveLength(1)
      expect(filtered[0].name).toBe('Capacitor')
    })

    it('should get unique categories', () => {
      const store = useLibraryStore()
      const testLibrary: ComponentLibrary = {
        id: 'test-lib',
        name: 'Test',
        version: '1.0.0',
        components: [
          { ...mockComponent, id: 'comp1', category: ComponentCategory.Passive },
          { ...mockComponent, id: 'comp2', category: ComponentCategory.Active },
          { ...mockComponent, id: 'comp3', category: ComponentCategory.Passive },
          { ...mockComponent, id: 'comp4', category: ComponentCategory.Connector }
        ]
      }
      
      store.loadLibrary(testLibrary)
      store.setActiveLibrary('test-lib')
      
      const categories = store.categories
      
      expect(categories).toHaveLength(3)
      expect(categories).toContain('Passive')
      expect(categories).toContain('Active')
      expect(categories).toContain('Digital')
    })
  })

  describe('Search', () => {
    it('should search components with relevance scoring', () => {
      const store = useLibraryStore()
      const testLibrary: ComponentLibrary = {
        id: 'test-lib',
        name: 'Test',
        version: '1.0.0',
        components: [
          { ...mockComponent, id: 'r1', name: 'Resistor', keywords: ['passive'] },
          { ...mockComponent, id: 'c1', name: 'Capacitor', description: 'A resistor-like component', keywords: ['passive'] },
          { ...mockComponent, id: 'l1', name: 'Inductor', keywords: ['resistor', 'passive'] }
        ]
      }
      
      store.loadLibrary(testLibrary)
      
      const results = store.searchComponents('resistor')
      
      expect(results.length).toBeGreaterThan(0)
      expect(results[0].component.name).toBe('Resistor') // Exact match should be first
      expect(results[0].relevance).toBe(100)
    })

    it('should search case-insensitive', () => {
      const store = useLibraryStore()
      const testLibrary: ComponentLibrary = {
        id: 'test-lib',
        name: 'Test',
        version: '1.0.0',
        components: [
          { ...mockComponent, name: 'UNIQUE_TEST_COMPONENT' }
        ]
      }
      
      store.loadLibrary(testLibrary)
      
      const results = store.searchComponents('unique_test_component')
      
      expect(results.length).toBeGreaterThan(0)
      expect(results.find(r => r.component.name === 'UNIQUE_TEST_COMPONENT')).toBeTruthy()
    })

    it('should clear search', () => {
      const store = useLibraryStore()
      store.searchQuery = 'test'
      store.selectedCategory = ComponentCategory.Passive
      
      store.clearSearch()
      
      expect(store.searchQuery).toBe('')
      expect(store.selectedCategory).toBeNull()
    })
  })

  describe('Favorites', () => {
    it('should toggle favorite status', () => {
      const store = useLibraryStore()
      
      expect(store.favoriteComponents.has('test-id')).toBe(false)
      
      store.toggleFavorite('test-id')
      expect(store.favoriteComponents.has('test-id')).toBe(true)
      
      store.toggleFavorite('test-id')
      expect(store.favoriteComponents.has('test-id')).toBe(false)
    })

    it('should get favorite components list', () => {
      const store = useLibraryStore()
      const testLibrary: ComponentLibrary = {
        id: 'test-lib',
        name: 'Test',
        version: '1.0.0',
        components: [
          { ...mockComponent, id: 'fav1' },
          { ...mockComponent, id: 'fav2' },
          { ...mockComponent, id: 'not-fav' }
        ]
      }
      
      store.loadLibrary(testLibrary)
      store.toggleFavorite('fav1')
      store.toggleFavorite('fav2')
      
      const favorites = store.favoriteComponentsList
      
      expect(favorites).toHaveLength(2)
      expect(favorites.find(c => c.id === 'fav1')).toBeTruthy()
      expect(favorites.find(c => c.id === 'fav2')).toBeTruthy()
      expect(favorites.find(c => c.id === 'not-fav')).toBeFalsy()
    })
  })

  describe('Recent Components', () => {
    it('should add component to recent list', () => {
      const store = useLibraryStore()
      const comp1 = { ...mockComponent, id: 'recent1' }
      
      store.addToRecent(comp1)
      
      expect(store.recentComponents).toHaveLength(1)
      expect(store.recentComponents[0].id).toBe('recent1')
    })

    it('should limit recent components to 20', () => {
      const store = useLibraryStore()
      const components = Array.from({ length: 25 }, (_, i) => ({
        ...mockComponent,
        id: `comp${i}`
      }))
      
      components.forEach(comp => store.addToRecent(comp))
      
      expect(store.recentComponents).toHaveLength(20)
      expect(store.recentComponents[0].id).toBe('comp24') // Most recent
      expect(store.recentComponents[19].id).toBe('comp5') // Oldest in list
    })

    it('should move existing component to top of recent list', () => {
      const store = useLibraryStore()
      const comp1 = { ...mockComponent, id: 'comp1' }
      const comp2 = { ...mockComponent, id: 'comp2' }
      const comp3 = { ...mockComponent, id: 'comp3' }
      
      store.addToRecent(comp1)
      store.addToRecent(comp2)
      store.addToRecent(comp3)
      store.addToRecent(comp1) // Add comp1 again
      
      expect(store.recentComponents[0].id).toBe('comp1')
      expect(store.recentComponents).toHaveLength(3)
    })
  })

  describe('Custom Components', () => {
    it('should create custom component', () => {
      const store = useLibraryStore()
      const customComponent = {
        ...mockComponent,
        id: 'custom-1',
        name: 'Custom Component'
      }
      
      store.createCustomComponent(customComponent)
      
      // Check that custom library was created
      const customLib = store.libraries.find(lib => lib.id === 'custom')
      expect(customLib).toBeTruthy()
      expect(customLib?.components).toHaveLength(1)
      expect(customLib?.components[0].id).toBe('custom-1')
    })

    it('should add multiple custom components to same library', () => {
      const store = useLibraryStore()
      const custom1 = { ...mockComponent, id: 'custom-1' }
      const custom2 = { ...mockComponent, id: 'custom-2' }
      
      store.createCustomComponent(custom1)
      store.createCustomComponent(custom2)
      
      const customLib = store.libraries.find(lib => lib.id === 'custom')
      expect(customLib?.components).toHaveLength(2)
    })
  })

  describe('Import/Export', () => {
    it('should export library as JSON', () => {
      const store = useLibraryStore()
      store.loadLibrary(mockLibrary)
      
      const exported = store.exportLibrary('test-library')
      const parsed = JSON.parse(exported)
      
      expect(parsed.id).toBe('test-library')
      expect(parsed.name).toBe('Test Library')
      expect(parsed.version).toBe('1.0.0')
      expect(parsed.components).toHaveLength(1)
    })

    it('should throw error when exporting non-existent library', () => {
      const store = useLibraryStore()
      
      expect(() => {
        store.exportLibrary('non-existent')
      }).toThrow('Library not found')
    })

    it('should import library from file', async () => {
      const store = useLibraryStore()
      
      const libraryData = {
        id: 'imported',
        name: 'Imported Library',
        version: '1.0.0',
        components: [
          { ...mockComponent, id: 'imported-1' }
        ]
      }
      
      const file = new File([JSON.stringify(libraryData)], 'library.json', {
        type: 'application/json'
      })
      
      const imported = await store.importLibrary(file)
      
      expect(imported.id).toBe('imported')
      expect(store.libraries.find(lib => lib.id === 'imported')).toBeTruthy()
    })

    it('should throw error for invalid library file', async () => {
      const store = useLibraryStore()
      
      const file = new File(['invalid json'], 'library.json', {
        type: 'application/json'
      })
      
      await expect(store.importLibrary(file)).rejects.toThrow('Invalid library file format')
    })
  })
})