<template>
  <div class="properties-panel">
    <div class="properties-header p-3 border-b border-gray-200 dark:border-gray-700">
      <h3 class="text-lg font-semibold">Properties</h3>
    </div>
    <div class="properties-content p-4">
      <n-empty v-if="!selectedComponent" description="No component selected" class="mt-8" />
      <n-form
        v-else
        :model="selectedComponent"
        label-placement="left"
        label-width="100"
        size="small"
      >
        <n-divider title-placement="left">General</n-divider>

        <n-form-item label="Name">
          <n-input
            v-model:value="selectedComponent.name"
            placeholder="Component name"
            @update:value="handlePropertyChange('name', $event)"
          />
        </n-form-item>

        <n-form-item label="Reference">
          <n-input
            v-model:value="selectedComponent.reference"
            placeholder="e.g., R1, C1, U1"
            @update:value="handlePropertyChange('reference', $event)"
          />
        </n-form-item>

        <n-form-item label="Value">
          <n-input
            v-model:value="selectedComponent.value"
            placeholder="e.g., 10k, 100nF"
            @update:value="handlePropertyChange('value', $event)"
          />
        </n-form-item>

        <n-form-item label="Type">
          <n-select
            v-model:value="selectedComponent.type"
            :options="componentTypeOptions"
            @update:value="handlePropertyChange('type', $event)"
          />
        </n-form-item>

        <n-divider title-placement="left">Position</n-divider>

        <n-form-item label="X Position">
          <n-input-number
            v-model:value="selectedComponent.x"
            :step="gridSize"
            :precision="0"
            @update:value="handlePropertyChange('x', $event || 0)"
          >
            <template #suffix>px</template>
          </n-input-number>
        </n-form-item>

        <n-form-item label="Y Position">
          <n-input-number
            v-model:value="selectedComponent.y"
            :step="gridSize"
            :precision="0"
            @update:value="handlePropertyChange('y', $event || 0)"
          >
            <template #suffix>px</template>
          </n-input-number>
        </n-form-item>

        <n-form-item label="Rotation">
          <n-input-number
            v-model:value="selectedComponent.rotation"
            :step="90"
            :min="0"
            :max="270"
            :precision="0"
            @update:value="handlePropertyChange('rotation', $event || 0)"
          >
            <template #suffix>°</template>
          </n-input-number>
        </n-form-item>

        <n-divider title-placement="left">Appearance</n-divider>

        <n-form-item label="Visible">
          <n-switch
            v-model:value="selectedComponent.visible"
            @update:value="handlePropertyChange('visible', $event)"
          />
        </n-form-item>

        <n-form-item label="Locked">
          <n-switch
            v-model:value="selectedComponent.locked"
            @update:value="handlePropertyChange('locked', $event)"
          />
        </n-form-item>

        <n-divider title-placement="left">Electrical</n-divider>

        <n-form-item label="Footprint">
          <n-input
            v-model:value="selectedComponent.footprint"
            placeholder="e.g., 0805, SOIC-8"
            @update:value="handlePropertyChange('footprint', $event)"
          />
        </n-form-item>

        <n-form-item label="Datasheet">
          <n-input
            v-model:value="selectedComponent.datasheet"
            placeholder="URL or file path"
            @update:value="handlePropertyChange('datasheet', $event)"
          />
        </n-form-item>

        <n-form-item label="Description">
          <n-input
            v-model:value="selectedComponent.description"
            type="textarea"
            :rows="3"
            placeholder="Component description"
            @update:value="handlePropertyChange('description', $event)"
          />
        </n-form-item>
      </n-form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import {
  NEmpty,
  NForm,
  NFormItem,
  NInput,
  NInputNumber,
  NSelect,
  NSwitch,
  NDivider
} from 'naive-ui'
import { useProjectStore, useUIStore } from '@/stores'

interface ComponentProperties {
  id: string
  name: string
  reference: string
  value: string
  type: string
  x: number
  y: number
  rotation: number
  visible: boolean
  locked: boolean
  footprint: string
  datasheet: string
  description: string
}

const projectStore = useProjectStore()
const uiStore = useUIStore()

const gridSize = computed(() => (uiStore.snapToGrid ? 10 : 1))

const selectedComponent = ref<ComponentProperties | null>(null)

const componentTypeOptions = [
  { label: 'Resistor', value: 'resistor' },
  { label: 'Capacitor', value: 'capacitor' },
  { label: 'Inductor', value: 'inductor' },
  { label: 'Diode', value: 'diode' },
  { label: 'Transistor', value: 'transistor' },
  { label: 'IC', value: 'ic' },
  { label: 'Connector', value: 'connector' },
  { label: 'Switch', value: 'switch' },
  { label: 'Crystal', value: 'crystal' },
  { label: 'Power', value: 'power' },
  { label: 'Ground', value: 'ground' },
  { label: 'Other', value: 'other' }
]

watch(
  () => projectStore.selectedComponentId,
  newId => {
    if (newId) {
      selectedComponent.value = {
        id: newId,
        name: 'Component',
        reference: 'R1',
        value: '10k',
        type: 'resistor',
        x: 100,
        y: 100,
        rotation: 0,
        visible: true,
        locked: false,
        footprint: '0805',
        datasheet: '',
        description: ''
      }
    } else {
      selectedComponent.value = null
    }
  }
)

const handlePropertyChange = (property: string, value: string | number | boolean) => {
  if (selectedComponent.value) {
    console.log(`Property ${property} changed to:`, value)
    projectStore.updateComponentProperty(selectedComponent.value.id, property, value)
  }
}
</script>

<style scoped>
.properties-panel {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: var(--n-color);
  border-left: 1px solid var(--n-border-color);
}

.properties-header {
  flex-shrink: 0;
}

.properties-content {
  flex: 1;
  overflow-y: auto;
}

:deep(.n-form-item) {
  margin-bottom: 12px;
}

:deep(.n-divider) {
  margin: 16px 0 12px 0;
}
</style>
