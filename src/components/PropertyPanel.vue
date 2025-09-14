<template>
  <div class="property-panel">
    <n-card title="属性面板" size="small">
      <n-tabs v-model:value="activeTab" type="line" animated>
        <!-- 元件属性 -->
        <n-tab-pane name="component" tab="元件" v-if="selectedComponent">
          <n-form
            ref="componentFormRef"
            :model="componentModel"
            :rules="componentRules"
            label-placement="left"
            label-width="80"
            size="small"
          >
            <n-form-item label="标识符" path="reference">
              <n-input
                v-model:value="componentModel.reference"
                placeholder="如: R1, C1, U1"
                @update:value="updateProperty('reference', $event)"
              />
            </n-form-item>
            
            <n-form-item label="值" path="value">
              <n-input
                v-model:value="componentModel.value"
                placeholder="如: 10kΩ, 100nF"
                @update:value="updateProperty('value', $event)"
              />
            </n-form-item>
            
            <n-form-item label="封装" path="footprint">
              <n-select
                v-model:value="componentModel.footprint"
                :options="footprintOptions"
                placeholder="选择封装"
                @update:value="updateProperty('footprint', $event)"
              />
            </n-form-item>
            
            <n-form-item label="位置 X" path="x">
              <n-input-number
                v-model:value="componentModel.x"
                :step="10"
                @update:value="updateProperty('x', $event)"
              />
            </n-form-item>
            
            <n-form-item label="位置 Y" path="y">
              <n-input-number
                v-model:value="componentModel.y"
                :step="10"
                @update:value="updateProperty('y', $event)"
              />
            </n-form-item>
            
            <n-form-item label="旋转" path="rotation">
              <n-slider
                v-model:value="componentModel.rotation"
                :min="0"
                :max="270"
                :step="90"
                :marks="rotationMarks"
                @update:value="updateProperty('rotation', $event)"
              />
            </n-form-item>
            
            <!-- 参数化属性 -->
            <n-divider title-placement="left">参数</n-divider>
            <template v-for="(_param, key) in componentModel.parameters" :key="key">
              <n-form-item :label="formatParamLabel(key)">
                <n-input
                  v-model:value="componentModel.parameters[key]"
                  @update:value="updateParameter(key, $event)"
                />
              </n-form-item>
            </template>
          </n-form>
        </n-tab-pane>
        
        <!-- 连线属性 -->
        <n-tab-pane name="wire" tab="连线" v-if="selectedWire">
          <n-form
            :model="wireModel"
            label-placement="left"
            label-width="80"
            size="small"
          >
            <n-form-item label="网络名" path="netName">
              <n-input
                v-model:value="wireModel.netName"
                placeholder="如: VCC, GND, SIG1"
                @update:value="updateWireProperty('netName', $event)"
              />
            </n-form-item>
            
            <n-form-item label="线宽" path="width">
              <n-select
                v-model:value="wireModel.width"
                :options="wireWidthOptions"
                @update:value="updateWireProperty('width', $event)"
              />
            </n-form-item>
            
            <n-form-item label="颜色" path="color">
              <n-color-picker
                v-model:value="wireModel.color"
                :modes="['hex']"
                @update:value="updateWireProperty('color', $event)"
              />
            </n-form-item>
            
            <n-form-item label="节点数">
              <n-tag type="info">{{ wireModel.points?.length || 0 }} 个节点</n-tag>
            </n-form-item>
          </n-form>
        </n-tab-pane>
        
        <!-- 标注属性 -->
        <n-tab-pane name="annotation" tab="标注">
          <n-form
            :model="annotationModel"
            label-placement="left"
            label-width="80"
            size="small"
          >
            <n-form-item label="文本">
              <n-input
                v-model:value="annotationModel.text"
                type="textarea"
                :rows="3"
                placeholder="输入标注文本"
                @update:value="updateAnnotation('text', $event)"
              />
            </n-form-item>
            
            <n-form-item label="字体大小">
              <n-input-number
                v-model:value="annotationModel.fontSize"
                :min="8"
                :max="72"
                @update:value="updateAnnotation('fontSize', $event)"
              />
            </n-form-item>
            
            <n-form-item label="字体">
              <n-select
                v-model:value="annotationModel.fontFamily"
                :options="fontOptions"
                @update:value="updateAnnotation('fontFamily', $event)"
              />
            </n-form-item>
            
            <n-form-item label="颜色">
              <n-color-picker
                v-model:value="annotationModel.color"
                :modes="['hex']"
                @update:value="updateAnnotation('color', $event)"
              />
            </n-form-item>
            
            <n-form-item label="对齐">
              <n-radio-group
                v-model:value="annotationModel.align"
                @update:value="updateAnnotation('align', $event)"
              >
                <n-radio-button value="left">左</n-radio-button>
                <n-radio-button value="center">中</n-radio-button>
                <n-radio-button value="right">右</n-radio-button>
              </n-radio-group>
            </n-form-item>
          </n-form>
        </n-tab-pane>
        
        <!-- 项目属性 -->
        <n-tab-pane name="project" tab="项目">
          <n-form
            :model="projectModel"
            label-placement="left"
            label-width="80"
            size="small"
          >
            <n-form-item label="名称">
              <n-input v-model:value="projectModel.name" />
            </n-form-item>
            
            <n-form-item label="作者">
              <n-input v-model:value="projectModel.author" />
            </n-form-item>
            
            <n-form-item label="版本">
              <n-input v-model:value="projectModel.version" />
            </n-form-item>
            
            <n-form-item label="描述">
              <n-input
                v-model:value="projectModel.description"
                type="textarea"
                :rows="3"
              />
            </n-form-item>
            
            <n-form-item label="创建时间">
              <n-date-picker
                v-model:value="projectModel.createdAt"
                type="datetime"
                disabled
              />
            </n-form-item>
            
            <n-form-item label="修改时间">
              <n-date-picker
                v-model:value="projectModel.updatedAt"
                type="datetime"
                disabled
              />
            </n-form-item>
          </n-form>
        </n-tab-pane>
      </n-tabs>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, reactive } from 'vue'
import {
  NCard,
  NTabs,
  NTabPane,
  NForm,
  NFormItem,
  NInput,
  NInputNumber,
  NSelect,
  NSlider,
  NColorPicker,
  NRadioGroup,
  NRadioButton,
  NDatePicker,
  NDivider,
  NTag,
  useMessage
} from 'naive-ui'
import type { SchematicComponent, SchematicWire } from '@/composables/useSchematicEditor'

const props = defineProps<{
  selectedComponent?: SchematicComponent | null
  selectedWire?: SchematicWire | null
  selectedAnnotation?: any | null
}>()

const emit = defineEmits<{
  updateComponent: [id: string, property: string, value: any]
  updateWire: [id: string, property: string, value: any]
  updateAnnotation: [id: string, property: string, value: any]
  updateProject: [property: string, value: any]
}>()

const message = useMessage()

// Tab control
const activeTab = ref('component')

// Component model
const componentModel = reactive({
  reference: '',
  value: '',
  footprint: '',
  x: 0,
  y: 0,
  rotation: 0,
  parameters: {} as Record<string, any>
})

// Wire model
const wireModel = reactive({
  netName: '',
  width: 2,
  color: '#000000',
  points: [] as Array<{ x: number; y: number }>
})

// Annotation model
const annotationModel = reactive({
  text: '',
  fontSize: 12,
  fontFamily: 'Arial',
  color: '#000000',
  align: 'left'
})

// Project model
const projectModel = reactive({
  name: 'Untitled Project',
  author: '',
  version: '1.0.0',
  description: '',
  createdAt: Date.now(),
  updatedAt: Date.now()
})

// Options
const footprintOptions = [
  { label: '0402', value: '0402' },
  { label: '0603', value: '0603' },
  { label: '0805', value: '0805' },
  { label: '1206', value: '1206' },
  { label: 'SOT-23', value: 'SOT-23' },
  { label: 'SOIC-8', value: 'SOIC-8' },
  { label: 'SOIC-14', value: 'SOIC-14' },
  { label: 'SOIC-16', value: 'SOIC-16' },
  { label: 'QFP-44', value: 'QFP-44' },
  { label: 'QFN-32', value: 'QFN-32' },
  { label: 'DIP-8', value: 'DIP-8' },
  { label: 'DIP-14', value: 'DIP-14' },
  { label: 'DIP-16', value: 'DIP-16' },
  { label: 'TO-92', value: 'TO-92' },
  { label: 'TO-220', value: 'TO-220' }
]

const wireWidthOptions = [
  { label: '细 (1px)', value: 1 },
  { label: '标准 (2px)', value: 2 },
  { label: '粗 (3px)', value: 3 },
  { label: '特粗 (4px)', value: 4 }
]

const fontOptions = [
  { label: 'Arial', value: 'Arial' },
  { label: 'Helvetica', value: 'Helvetica' },
  { label: 'Times New Roman', value: 'Times New Roman' },
  { label: 'Courier New', value: 'Courier New' },
  { label: 'Verdana', value: 'Verdana' },
  { label: 'Georgia', value: 'Georgia' },
  { label: 'monospace', value: 'monospace' }
]

const rotationMarks = {
  0: '0°',
  90: '90°',
  180: '180°',
  270: '270°'
}

// Validation rules
const componentRules = {
  reference: {
    required: true,
    message: '请输入元件标识符',
    trigger: 'blur'
  },
  value: {
    required: true,
    message: '请输入元件值',
    trigger: 'blur'
  }
}

// Watch for selected component changes
watch(() => props.selectedComponent, (newComponent) => {
  if (newComponent) {
    componentModel.reference = newComponent.reference || ''
    componentModel.value = newComponent.value || ''
    componentModel.footprint = (newComponent as any).footprint || ''
    componentModel.x = newComponent.x
    componentModel.y = newComponent.y
    componentModel.rotation = newComponent.rotation
    componentModel.parameters = (newComponent as any).parameters || {}
    activeTab.value = 'component'
  }
}, { immediate: true })

// Watch for selected wire changes
watch(() => props.selectedWire, (newWire) => {
  if (newWire) {
    wireModel.netName = newWire.netName || ''
    wireModel.width = (newWire as any).width || 2
    wireModel.color = (newWire as any).color || '#000000'
    wireModel.points = newWire.points
    activeTab.value = 'wire'
  }
}, { immediate: true })

// Update functions
function updateProperty(property: string, value: any) {
  if (props.selectedComponent) {
    emit('updateComponent', props.selectedComponent.id, property, value)
  }
}

function updateParameter(key: string, value: any) {
  if (props.selectedComponent) {
    const parameters = { ...componentModel.parameters, [key]: value }
    emit('updateComponent', props.selectedComponent.id, 'parameters', parameters)
  }
}

function updateWireProperty(property: string, value: any) {
  if (props.selectedWire) {
    emit('updateWire', props.selectedWire.id, property, value)
  }
}

function updateAnnotation(property: string, value: any) {
  if (props.selectedAnnotation) {
    emit('updateAnnotation', props.selectedAnnotation.id, property, value)
  }
}

function formatParamLabel(key: string): string {
  // Convert snake_case or camelCase to readable label
  return key
    .replace(/_/g, ' ')
    .replace(/([A-Z])/g, ' $1')
    .trim()
    .split(' ')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join(' ')
}
</script>

<style scoped>
.property-panel {
  width: 100%;
  height: 100%;
  padding: 16px;
  overflow-y: auto;
}

:deep(.n-card) {
  height: 100%;
}

:deep(.n-tabs) {
  height: calc(100% - 40px);
}

:deep(.n-tab-pane) {
  padding: 12px 0;
  height: 100%;
  overflow-y: auto;
}

:deep(.n-form-item) {
  margin-bottom: 12px;
}

:deep(.n-divider) {
  margin: 16px 0 12px;
}
</style>