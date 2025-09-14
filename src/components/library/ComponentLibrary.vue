<template>
  <div class="component-library">
    <div class="library-header">
      <n-input
        v-model:value="searchQuery"
        placeholder="Search components..."
        clearable
        @update:value="handleSearch"
      >
        <template #prefix>
          <n-icon :component="SearchOutline" />
        </template>
      </n-input>
      
      <n-space class="library-actions">
        <n-dropdown :options="libraryOptions" @select="handleLibrarySelect">
          <n-button size="small">
            {{ currentLibraryName }}
            <template #icon>
              <n-icon :component="ChevronDownOutline" />
            </template>
          </n-button>
        </n-dropdown>
        
        <n-button size="small" @click="showImportDialog = true">
          <template #icon>
            <n-icon :component="CloudDownloadOutline" />
          </template>
        </n-button>
      </n-space>
    </div>

    <n-tabs v-model:value="activeTab" type="line" size="small">
      <n-tab-pane name="categories" tab="Categories">
        <div class="category-filter">
          <n-radio-group v-model:value="selectedCategory" size="small">
            <n-radio-button :value="undefined">All</n-radio-button>
            <n-radio-button 
              v-for="cat in categories" 
              :key="cat.id"
              :value="cat.id"
            >
              {{ formatCategoryName(cat) }}
            </n-radio-button>
          </n-radio-group>
        </div>
        
        <n-scrollbar class="component-grid-container">
          <div class="component-grid">
            <ComponentPreview
              v-for="component in filteredComponents"
              :key="component.id"
              :component="component"
              :show-favorite="true"
              @click="selectComponent"
              @start-drag="handleStartDrag"
            />
          </div>
        </n-scrollbar>
      </n-tab-pane>
      
      <n-tab-pane name="recent" tab="Recent">
        <n-scrollbar class="component-grid-container">
          <div class="component-grid">
            <ComponentPreview
              v-for="component in recentComponents"
              :key="component.id"
              :component="component"
              :show-favorite="true"
              @click="selectComponent"
              @start-drag="handleStartDrag"
            />
          </div>
          <n-empty v-if="recentComponents.length === 0" description="No recent components" />
        </n-scrollbar>
      </n-tab-pane>
      
      <n-tab-pane name="favorites" tab="Favorites">
        <n-scrollbar class="component-grid-container">
          <div class="component-grid">
            <ComponentPreview
              v-for="component in favoriteComponents"
              :key="component.id"
              :component="component"
              :show-favorite="true"
              @click="selectComponent"
              @start-drag="handleStartDrag"
            />
          </div>
          <n-empty v-if="favoriteComponents.length === 0" description="No favorite components" />
        </n-scrollbar>
      </n-tab-pane>
    </n-tabs>
    
    <div v-if="selectedComponent" class="component-details">
      <n-divider />
      <h4>{{ selectedComponent.name }}</h4>
      <p class="component-description">{{ selectedComponent.description }}</p>
      
      <n-descriptions :column="1" size="small">
        <n-descriptions-item v-if="selectedComponent.footprint" label="Footprint">
          {{ selectedComponent.footprint }}
        </n-descriptions-item>
        <n-descriptions-item label="Category">
          {{ selectedComponent.categoryId }}
        </n-descriptions-item>
        <n-descriptions-item label="Pins">
          {{ selectedComponent.pins.length }}
        </n-descriptions-item>
      </n-descriptions>
      
      <n-space>
        <n-button size="small" type="primary" @click="addToSchematic">
          Add to Schematic
        </n-button>
        <n-button v-if="selectedComponent.keywords?.length" size="small">
          Keywords: {{ selectedComponent.keywords.join(', ') }}
        </n-button>
      </n-space>
    </div>
    
    <!-- Import Library Dialog -->
    <n-modal
      v-model:show="showImportDialog"
      preset="dialog"
      title="Import Component Library"
      positive-text="Import"
      negative-text="Cancel"
      @positive-click="handleImport"
    >
      <n-upload
        v-model:file-list="importFileList"
        :max="1"
        accept=".json,.lib"
        @change="handleFileChange"
      >
        <n-upload-dragger>
          <div style="margin-bottom: 12px">
            <n-icon size="48" :depth="3" :component="CloudUploadOutline" />
          </div>
          <n-text style="font-size: 16px">
            Click or drag library file here
          </n-text>
          <n-p depth="3" style="margin: 8px 0 0 0">
            Supports JSON library format
          </n-p>
        </n-upload-dragger>
      </n-upload>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import {
  NInput,
  NIcon,
  NSpace,
  NButton,
  NDropdown,
  NTabs,
  NTabPane,
  NRadioGroup,
  NRadioButton,
  NScrollbar,
  NDivider,
  NDescriptions,
  NDescriptionsItem,
  NEmpty,
  NModal,
  NUpload,
  NUploadDragger,
  NText,
  NP,
  useMessage,
  type DropdownOption,
  type UploadFileInfo
} from 'naive-ui'
import {
  SearchOutline,
  ChevronDownOutline,
  CloudDownloadOutline,
  CloudUploadOutline
} from '@vicons/ionicons5'
import ComponentPreview from './ComponentPreview.vue'
import { useLibraryStore } from '@/stores/library'
import type { ComponentTemplate, ComponentCategory } from '@/api/library'

const emit = defineEmits<{
  selectComponent: [component: ComponentTemplate]
  addComponent: [component: ComponentTemplate]
}>()

const message = useMessage()
const libraryStore = useLibraryStore()

const searchQuery = ref('')
const activeTab = ref('categories')
const selectedCategory = ref<ComponentCategory | null>(null)
const selectedComponent = ref<ComponentTemplate | null>(null)
const showImportDialog = ref(false)
const importFileList = ref<UploadFileInfo[]>([])

// Sync with store
watch(() => libraryStore.searchQuery, (val) => {
  searchQuery.value = val
})

watch(() => libraryStore.selectedCategory, (val) => {
  selectedCategory.value = val
})

const filteredComponents = computed(() => libraryStore.filteredComponents)
const recentComponents = computed(() => libraryStore.recentComponents)
const favoriteComponents = computed(() => libraryStore.favoriteComponentsList)
const categories = computed(() => libraryStore.categories)

const currentLibraryName = computed(() => 
  libraryStore.activeLibrary?.name || 'No Library'
)

const libraryOptions = computed<DropdownOption[]>(() => 
  libraryStore.libraries.map(lib => ({
    label: lib.name,
    key: lib.id,
    disabled: lib.id === libraryStore.activeLibraryId
  }))
)

function handleSearch(value: string) {
  libraryStore.searchQuery = value
}

function handleLibrarySelect(key: string) {
  libraryStore.setActiveLibrary(key)
}

function selectComponent(component: ComponentTemplate) {
  selectedComponent.value = component
  emit('selectComponent', component)
}

function handleStartDrag(component: ComponentTemplate) {
  libraryStore.addToRecent(component)
}

function addToSchematic() {
  if (selectedComponent.value) {
    emit('addComponent', selectedComponent.value)
    libraryStore.addToRecent(selectedComponent.value)
  }
}


function formatCategoryName(category: ComponentCategory | string | null): string {
  if (!category) return 'All'
  if (typeof category === 'object' && 'name' in category) {
    return category.name
  }
  const names: Record<string, string> = {
    passive: 'Passive',
    active: 'Active',
    ic: 'ICs',
    connector: 'Connectors',
    power: 'Power',
    mechanical: 'Mechanical',
    custom: 'Custom'
  }
  return names[category as string] || category as string
}

async function handleFileChange(options: { fileList: UploadFileInfo[] }) {
  importFileList.value = options.fileList
}

async function handleImport() {
  if (importFileList.value.length === 0) {
    message.warning('Please select a library file')
    return
  }
  
  const file = importFileList.value[0].file
  if (!file) {
    message.error('Invalid file')
    return
  }
  
  try {
    const library = await libraryStore.importLibrary(file)
    message.success(`Imported library: ${library.name}`)
    showImportDialog.value = false
    importFileList.value = []
  } catch (error) {
    message.error(`Failed to import library: ${error}`)
  }
}

// Update selected category in store
watch(selectedCategory, (val) => {
  libraryStore.selectedCategory = val
})

// Initialize store on mount
onMounted(async () => {
  await libraryStore.initialize()
})
</script>

<style scoped>
.component-library {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 12px;
  background: white;
}

.library-header {
  margin-bottom: 12px;
}

.library-actions {
  margin-top: 8px;
  display: flex;
  justify-content: space-between;
}

.category-filter {
  margin-bottom: 12px;
  padding: 8px 0;
}

.category-filter :deep(.n-radio-group) {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.component-grid-container {
  flex: 1;
  min-height: 0;
}

.component-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 8px;
  padding: 4px;
}

.component-details {
  padding-top: 12px;
  border-top: 1px solid var(--border-color);
}

.component-details h4 {
  margin: 0 0 8px 0;
  font-size: 14px;
  font-weight: 500;
}

.component-description {
  margin: 0 0 12px 0;
  font-size: 12px;
  color: var(--text-secondary);
}

:deep(.n-tabs) {
  flex: 1;
  display: flex;
  flex-direction: column;
}

:deep(.n-tabs-pane-wrapper) {
  flex: 1;
  min-height: 0;
}

:deep(.n-tab-pane) {
  height: 100%;
  padding: 0 !important;
}
</style>