<template>
  <div class="sidebar">
    <div class="sidebar-header p-3 border-b border-gray-200 dark:border-gray-700">
      <h3 class="text-lg font-semibold">Project Explorer</h3>
    </div>
    <div class="sidebar-content p-2">
      <n-tree
        v-if="treeData.length > 0"
        :data="treeData"
        :default-expanded-keys="expandedKeys"
        :selected-keys="selectedKeys"
        :render-prefix="renderPrefix"
        :node-props="nodeProps"
        block-line
        selectable
        @update:selected-keys="handleSelectNode"
        @update:expanded-keys="handleExpandedKeys"
      />
      <n-empty v-else description="No project opened" class="mt-8" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, h } from 'vue'
import { NTree, NEmpty, NIcon, NDropdown } from 'naive-ui'
import type { TreeOption } from 'naive-ui'
import {
  FolderOpenOutline,
  FolderOutline,
  DocumentOutline,
  HardwareChipOutline,
  GridOutline,
  LibraryOutline
} from '@vicons/ionicons5'
import { useProjectStore } from '@/stores'

const projectStore = useProjectStore()

const expandedKeys = ref<string[]>(['project-root'])
const selectedKeys = ref<string[]>([])

const treeData = computed<TreeOption[]>(() => {
  if (!projectStore.currentProject) {
    return []
  }

  return [
    {
      key: 'project-root',
      label: projectStore.currentProject.name,
      children: [
        {
          key: 'schematics',
          label: 'Schematics',
          children: [
            {
              key: 'main-schematic',
              label: 'main.sch',
              isLeaf: true,
              type: 'schematic'
            }
          ]
        },
        {
          key: 'pcb',
          label: 'PCB Layouts',
          children: [
            {
              key: 'main-pcb',
              label: 'main.pcb',
              isLeaf: true,
              type: 'pcb'
            }
          ]
        },
        {
          key: 'libraries',
          label: 'Libraries',
          children: [
            {
              key: 'components-lib',
              label: 'components.lib',
              isLeaf: true,
              type: 'library'
            }
          ]
        },
        {
          key: 'outputs',
          label: 'Outputs',
          children: [
            {
              key: 'gerbers',
              label: 'Gerbers',
              children: []
            },
            {
              key: 'bom',
              label: 'BOM',
              isLeaf: true,
              type: 'document'
            }
          ]
        }
      ]
    }
  ]
})

const renderPrefix = ({ option }: { option: TreeOption }) => {
  const iconMap: Record<string, typeof HardwareChipOutline> = {
    schematic: HardwareChipOutline,
    pcb: GridOutline,
    library: LibraryOutline,
    document: DocumentOutline
  }

  if (option.type && iconMap[option.type as string]) {
    return h(NIcon, null, { default: () => h(iconMap[option.type as string]) })
  }

  if (option.children && option.children.length > 0) {
    const isExpanded = expandedKeys.value.includes(option.key as string)
    return h(NIcon, null, {
      default: () => h(isExpanded ? FolderOpenOutline : FolderOutline)
    })
  }

  return h(NIcon, null, { default: () => h(DocumentOutline) })
}

const nodeProps = ({ option }: { option: TreeOption }) => {
  return {
    onContextmenu: (e: MouseEvent) => {
      e.preventDefault()
      handleContextMenu(option, e)
    }
  }
}

const handleSelectNode = (keys: string[]) => {
  selectedKeys.value = keys
  if (keys.length > 0) {
    const key = keys[0]
    if (key === 'main-schematic') {
      projectStore.openSchematic()
    } else if (key === 'main-pcb') {
      projectStore.openPCB()
    }
  }
}

const handleExpandedKeys = (keys: string[]) => {
  expandedKeys.value = keys
}

const handleContextMenu = (option: TreeOption, e: MouseEvent) => {
  const dropdownOptions = [
    {
      label: 'New File',
      key: 'new-file'
    },
    {
      label: 'Rename',
      key: 'rename'
    },
    {
      type: 'divider',
      key: 'd1'
    },
    {
      label: 'Delete',
      key: 'delete'
    }
  ]

  NDropdown.show({
    options: dropdownOptions,
    x: e.clientX,
    y: e.clientY,
    onSelect: (key: string) => {
      handleMenuAction(key, option)
    }
  })
}

const handleMenuAction = (action: string, option: TreeOption) => {
  switch (action) {
    case 'new-file':
      console.log('Creating new file in', option.label)
      break
    case 'rename':
      console.log('Renaming', option.label)
      break
    case 'delete':
      console.log('Deleting', option.label)
      break
  }
}
</script>

<style scoped>
.sidebar {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background-color: var(--n-color);
  border-right: 1px solid var(--n-border-color);
}

.sidebar-header {
  flex-shrink: 0;
}

.sidebar-content {
  flex: 1;
  overflow-y: auto;
}
</style>
