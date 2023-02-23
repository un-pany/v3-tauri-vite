<script setup lang="ts">
import { ref } from "vue"
import { useRouter } from "vue-router"
import { useUserStore } from "@/store/modules/user"

const router = useRouter()
const userStore = useUserStore()

const greetMsg = ref("Hello!")
const getGreet = async () => {
  greetMsg.value = await window.__TAURI__.invoke("greet", { name: userStore.username })
}
getGreet()

const onBack = () => {
  userStore.doLogout()
  router.push({ path: "/login" })
}

// end
</script>

<template>
  <div class="container">
    <h1>{{ greetMsg }}</h1>
    <el-button size="large" type="primary" @click="onBack()">返回</el-button>
  </div>
</template>

<style lang="scss" scoped></style>
