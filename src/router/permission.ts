import router from "."
import { getToken } from "@/utils/cache/sessionStorage"
import { whiteList } from "@/config/white-list"

router.beforeEach(async (to, _from, next) => {
  // 判断该用户是否登录
  if (getToken()) {
    if (to.path === "/login") {
      // 如果已经登录，并准备进入 Login 页面，则重定向到主页
      next({ path: "/" })
    } else {
      next()
    }
  } else {
    // 如果没有 Token
    if (whiteList.indexOf(to.path) !== -1) {
      // 如果在免登录的白名单中，则直接进入
      next()
    } else {
      // 其他没有访问权限的页面将被重定向到登录页面
      next("/login")
    }
  }
})
