import { defineConfig } from "vite"
import { resolve } from "path"
import vue from "@vitejs/plugin-vue"

const isDebug = process.env.TAURI_DEBUG === "true"

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: true,
  resolve: {
    alias: {
      "@": resolve(__dirname, "./src") // 路径别名
    }
  },
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    // produce sourcemaps for debug builds
    sourcemap: isDebug,
    /** 消除打包大小超过 500kb 警告 */
    chunkSizeWarningLimit: 2000,
    /** vite 2.6.x 以上需要配置 minify: terser，terserOptions 才能生效 */
    minify: isDebug ? false : "terser",
    /** 在 build 代码时移除 console.log、debugger 和 注释 */
    terserOptions: isDebug
      ? null
      : {
          compress: {
            drop_console: false,
            drop_debugger: true,
            pure_funcs: ["console.log"]
          },
          output: {
            /** 删除注释 */
            comments: false
          }
        }
  }
})
