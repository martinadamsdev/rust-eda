import { useMessage } from 'naive-ui'

export interface AppError {
  code: string
  message: string
  details?: any
  timestamp: Date
  severity: 'low' | 'medium' | 'high' | 'critical'
  context?: string
  userAction?: string
}

export class ErrorHandler {
  private static errors: AppError[] = []
  private static maxErrors = 100

  static handle(
    error: Error | string, 
    code?: string,
    severity: 'low' | 'medium' | 'high' | 'critical' = 'medium',
    context?: string
  ): void {
    const appError: AppError = {
      code: code || 'UNKNOWN_ERROR',
      message: typeof error === 'string' ? error : error.message,
      details: error instanceof Error ? error.stack : undefined,
      timestamp: new Date(),
      severity,
      context
    }

    this.errors.push(appError)

    if (this.errors.length > this.maxErrors) {
      this.errors.shift()
    }

    console.error('[ErrorHandler]', appError)

    if (typeof window !== 'undefined') {
      const message = useMessage()
      message.error(appError.message)
    }
  }

  static getErrors(): AppError[] {
    return [...this.errors]
  }

  static clearErrors(): void {
    this.errors = []
  }

  static async withErrorHandling<T>(
    fn: () => Promise<T>,
    errorMessage?: string
  ): Promise<T | null> {
    try {
      return await fn()
    } catch (error) {
      this.handle(error as Error, errorMessage || 'Operation failed')
      return null
    }
  }
}

export function createErrorHandler(context: string) {
  return (error: Error | string) => {
    ErrorHandler.handle(error, context)
  }
}

export function handleTauriError(error: any): void {
  if (typeof error === 'string') {
    ErrorHandler.handle(error, 'TAURI_ERROR')
  } else if (error?.message) {
    ErrorHandler.handle(error.message, 'TAURI_ERROR')
  } else {
    ErrorHandler.handle('Unknown Tauri error', 'TAURI_ERROR')
  }
}

export function handleValidationError(field: string, message: string): void {
  ErrorHandler.handle(`${field}: ${message}`, 'VALIDATION_ERROR')
}

export function handleNetworkError(url: string, status?: number): void {
  const message = status
    ? `Network request to ${url} failed with status ${status}`
    : `Network request to ${url} failed`
  ErrorHandler.handle(message, 'NETWORK_ERROR')
}

export function setupGlobalErrorHandlers(): void {
  if (typeof window !== 'undefined') {
    window.addEventListener('error', event => {
      ErrorHandler.handle(event.error || event.message.toString(), 'WINDOW_ERROR')
    })

    window.addEventListener('unhandledrejection', event => {
      ErrorHandler.handle(
        event.reason?.message || event.reason || 'Unhandled promise rejection',
        'UNHANDLED_REJECTION'
      )
    })
  }
}
