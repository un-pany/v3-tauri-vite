import store from ".."
import { ref } from "vue"
import { defineStore } from "pinia"
import { loginApi } from "@/api/login"
import { getToken, setToken, removeToken } from "@/utils/cache/sessionStorage"

export const useUserStore = defineStore("user", () => {
  const token = ref<string>(getToken() || "")
  const username = ref<string>("")

  /** 登录 */
  const doLogin = (name: string) => {
    return new Promise((resolve, reject) => {
      if (!name) {
        return reject(false)
      }
      loginApi()
        .then((res) => {
          setToken(res.data.token)
          token.value = res.data.token
          username.value = name
          resolve(true)
        })
        .catch(() => reject(false))
    })
  }

  /** 登出 */
  const doLogout = () => {
    removeToken()
    token.value = ""
    username.value = ""
  }

  return { token, username, doLogin, doLogout }
})

/** 在 setup 外使用 */
export function useUserStoreHook() {
  return useUserStore(store)
}
