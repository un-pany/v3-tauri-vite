<script setup lang="ts">
import { ref } from "vue"
import { useRouter } from "vue-router"
import { useUserStore } from "@/store/modules/user"

const router = useRouter()
const userStore = useUserStore()

const loading = ref(false)
const name = ref("admin")

const onLogin = async () => {
  if (loading.value || !name.value) return
  loading.value = true
  const flag = await userStore.doLogin(name.value).catch(() => false)
  flag && router.push({ path: "/" })
  loading.value = false
}

// end
</script>

<template>
  <div v-loading="loading" class="container">
    <div class="title">
      <img src="/tauri.svg" alt="Tauri logo" />
      <span>Welcome to Tauri!</span>
    </div>
    <el-input v-model="name" size="large" placeholder="Enter a name..." @keyup.enter="onLogin()" />
    <el-button size="large" type="primary" :disabled="!name" @click="onLogin()">确定</el-button>
  </div>
</template>

<style lang="scss" scoped>
.title {
  margin-bottom: 10vh;
  display: flex;
  align-items: center;
  justify-content: center;

  img {
    width: 80px;
    height: 80px;
    padding: 0 20px;
    transition: 0.75s;
    will-change: filter;
    &:hover {
      filter: drop-shadow(0 0 2em #24c8db);
    }
  }

  span {
    font-size: 2em;
    font-weight: bolder;
  }
}

.el-input {
  width: auto;
  margin-right: 20px;
}
</style>
