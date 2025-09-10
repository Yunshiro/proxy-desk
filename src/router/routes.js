import { createRouter, createWebHashHistory } from 'vue-router'
import ProxyView from '../pages/proxy/ProxyView.vue'
import ConfCenter from '../pages/conf-center/ConfCenter.vue'


const routes = [
  { path: '/', component: ProxyView },
  { path: '/confcenter', component: ConfCenter },
]

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
})