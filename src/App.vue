<script lang="ts" setup>
import zhCn from "element-plus/lib/locale/lang/zh-cn"
import { onBeforeUnmount, onMounted } from "vue"

/** 将 Element Plus 的语言设置为中文 */
const locale = zhCn

const env = import.meta.env
const DevEnv = env.DEV || env.TAURI_DEBUG === "true"

const contextmenu = (event: MouseEvent) => {
  !DevEnv && event.preventDefault()
}

onMounted(() => {
  document.addEventListener("contextmenu", contextmenu)
})
onBeforeUnmount(() => {
  document.removeEventListener("contextmenu", contextmenu)
})

// end
</script>

<template>
  <ElConfigProvider :locale="locale">
    <router-view />
  </ElConfigProvider>
</template>
