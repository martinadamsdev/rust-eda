import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: () => import('@/views/HomeView.vue')
  },
  {
    path: '/editor',
    name: 'editor',
    component: () => import('@/views/EditorView.vue'),
    children: [
      {
        path: 'schematic',
        name: 'schematic-editor',
        component: () => import('@/views/SchematicEditor.vue')
      },
      {
        path: 'pcb',
        name: 'pcb-editor',
        component: () => import('@/views/PCBEditor.vue')
      },
      {
        path: 'simulation',
        name: 'simulation',
        component: () => import('@/views/SimulationView.vue')
      }
    ]
  },
  {
    path: '/library',
    name: 'library',
    component: () => import('@/views/LibraryView.vue')
  },
  {
    path: '/settings',
    name: 'settings',
    component: () => import('@/views/SettingsView.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

router.beforeEach((to, from, next) => {
  console.log(`Navigating from ${from.path} to ${to.path}`)
  next()
})

router.afterEach(() => {
  console.log('Navigation complete')
})

export default router
