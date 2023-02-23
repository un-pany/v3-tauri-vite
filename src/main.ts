// core
import { createApp } from "vue"
import App from "@/App.vue"
import store from "@/store"
import router from "@/router"
import "@/router/permission"
// load
import { loadPlugins } from "@/plugins"
// css
import "@/styles/index.scss"
import "normalize.css"
import "element-plus/dist/index.css"
import "element-plus/theme-chalk/dark/css-vars.css"

const app = createApp(App)

/** 加载插件 */
loadPlugins(app)

app.use(store).use(router).mount("#app")
