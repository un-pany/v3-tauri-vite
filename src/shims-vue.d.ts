/// <reference types="vite/client" />

declare module '*.vue' {
    import { DefineComponent } from 'vue'
    // eslint-disable-next-line @typescript-eslint/no-explicit-any, @typescript-eslint/ban-types
    const component: DefineComponent<{}, {}, any>
    export default component
}

declare module 'virtual:*' {
    const result: any
    export default result
}

interface Window {
  __TAURI__: typeof import('@tauri-apps/api')
}
