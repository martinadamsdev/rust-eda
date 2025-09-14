import js from '@eslint/js'
import vue from 'eslint-plugin-vue'
import tseslint from 'typescript-eslint'

export default tseslint.config(
  // 忽略的文件和目录
  {
    ignores: ['dist', 'src-tauri', 'node_modules', '*.config.js', '*.config.ts', '.vscode', 'docs']
  },

  // JavaScript 推荐规则
  js.configs.recommended,

  // TypeScript 推荐规则
  ...tseslint.configs.recommended,

  // Vue 3 推荐规则
  ...vue.configs['flat/recommended'],

  // 自定义规则 - 仅代码质量，不涉及格式
  {
    files: ['**/*.{js,jsx,ts,tsx,vue}'],
    languageOptions: {
      globals: {
        console: 'readonly',
        window: 'readonly',
        document: 'readonly',
        requestAnimationFrame: 'readonly',
        cancelAnimationFrame: 'readonly',
        setTimeout: 'readonly',
        clearTimeout: 'readonly',
        setInterval: 'readonly',
        clearInterval: 'readonly',
        NodeJS: 'readonly',
        global: 'readonly'
      },
      parserOptions: {
        parser: tseslint.parser,
        project: './tsconfig.json',
        extraFileExtensions: ['.vue']
      }
    },
    rules: {
      // 代码质量规则
      'no-console': 'warn',
      'no-debugger': 'error',
      'no-unused-vars': 'off', // 使用 TypeScript 的规则
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_'
        }
      ],
      '@typescript-eslint/no-explicit-any': 'warn',
      'prefer-const': 'error',
      'no-var': 'error',

      // Vue 代码质量规则
      'vue/no-unused-components': 'warn',
      'vue/no-unused-vars': 'warn',
      'vue/require-v-for-key': 'error',
      'vue/valid-v-for': 'error',
      'vue/no-use-v-if-with-v-for': 'error',
      'vue/multi-word-component-names': 'off',
      
      // Other code quality rules
      'no-case-declarations': 'off',
      '@typescript-eslint/no-this-alias': 'off',
      'no-prototype-builtins': 'off',
      'no-async-promise-executor': 'off',
      '@typescript-eslint/no-empty-object-type': 'off',

      // 关闭所有格式相关的规则（交给 Prettier）
      indent: 'off',
      quotes: 'off',
      semi: 'off',
      'comma-dangle': 'off',
      'max-len': 'off',
      'vue/html-indent': 'off',
      'vue/max-attributes-per-line': 'off',
      'vue/html-closing-bracket-newline': 'off',
      'vue/multiline-html-element-content-newline': 'off',
      'vue/singleline-html-element-content-newline': 'off',
      'vue/html-self-closing': 'off'
    }
  }
)
