<script setup>
import { onMounted, ref } from 'vue'

import { invoke } from '@tauri-apps/api/core'

const proxyStatus = ref('no')

const setGitProxy = () => {
  invoke("set_http_proxy", { proxyUrl: "",isHttps: false })
}

const showDialog = ref(false)

const confs = ref([])
const initConfigs = async () => {
  const result = await invoke('query_proxy_configs')
  confs.value = result
}

onMounted(() => {
  initConfigs()
})

</script>

<template>
  <div id="proxy-view" style="background-color: antiquewhite; height: 100vh;">
    <div id="card-container">
      <v-card id="proxy-card" width="150px" v-for="conf in confs" hover @click="showDialog = true">
        <v-card-item>
          <v-card-title>
            {{ conf.title }}
          </v-card-title>
          <v-card-subtitle>
            {{ conf.proxy_url }}
          </v-card-subtitle>
        </v-card-item>
        <v-card-actions class="position-absolute top-0 right-0">
          <v-switch @click.stop="setGitProxy" color="primary" v-model="conf.status"
            false-value="inactive" true-value="active"
          ></v-switch>
          {{ conf.status }}
        </v-card-actions>
      </v-card>
    </div>

  </div>

  <v-dialog v-model="showDialog" width="auto">
    <v-card max-width="400" prepend-icon="mdi-update" text="1111 complete." title="Update in progress">
      <template v-slot:actions>
        <v-btn class="ms-auto" text="Ok" @click="showDialog = false"></v-btn>
      </template>
    </v-card>
  </v-dialog>

</template>

<style scoped>
#card-container {
  display: flex;
  flex-wrap: wrap;
  gap: 7px;
  width: 100%;
  margin: 0 auto;
  position: relative;
  left: 10px;
  top: 10px;
}

#proxy-card {
  flex: 1 1 calc(25% -10px);
  min-width: calc(33.333% - 12px);
}
</style>