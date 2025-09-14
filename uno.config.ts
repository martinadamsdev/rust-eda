import { defineConfig } from 'unocss'
import presetWind4 from '@unocss/preset-wind4'

export default defineConfig({
  presets: [
    presetWind4({
      preflights: {
        reset: true
      }
    })
  ],
  theme: {
    colors: {
      primary: '#18a058',
      secondary: '#2080f0',
      success: '#18a058',
      warning: '#f0a020',
      error: '#d03050',
      info: '#2080f0'
    }
  },
  shortcuts: {
    btn: 'px-4 py-2 rounded inline-block bg-primary text-white cursor-pointer hover:bg-primary-600 disabled:cursor-default disabled:bg-gray-600 disabled:opacity-50',
    'icon-btn':
      'text-[0.9em] inline-block cursor-pointer select-none opacity-75 transition duration-200 ease-in-out hover:opacity-100 hover:text-primary !outline-none'
  }
})
