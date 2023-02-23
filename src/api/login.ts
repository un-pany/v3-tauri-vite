import { request } from "@/utils/service"

/** 登录并返回 Token */
export function loginApi() {
  return request({
    url: "/users/login",
    method: "post",
    data: {
      username: "admin",
      password: "123456",
      code: "6"
    }
  })
}
