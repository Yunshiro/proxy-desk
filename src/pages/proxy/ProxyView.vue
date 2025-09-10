<script setup>
import { ref } from 'vue'

import { invoke } from '@tauri-apps/api/core'

const setGitProxy = () => {
  invoke("set_http_proxy")
}

const cards = [
  { "title": "Title1", "subtitle": "subtitle1" },
  { "title": "Title2", "subtitle": "subtitle2" },
  { "title": "Title3", "subtitle": "subtitle3" },
  { "title": "Title4", "subtitle": "subtitle4" },
  { "title": "Title5", "subtitle": "subtitle5" },
  { "title": "Title6", "subtitle": "subtitle6" },
  { "title": "Title7", "subtitle": "subtitle7" },
  { "title": "Title8", "subtitle": "subtitle8" },
]

const showDialog = ref(false)

</script>

<template>
  <div id="proxy-view" style="background-color: antiquewhite; height: 100vh;">
    <div id="card-container">
      <v-card id="proxy-card" width="150px" v-for="cd in cards" hover @click="showDialog = true">
        <v-card-item>
          <v-card-title>
            {{ cd.title }}
          </v-card-title>
          <v-card-subtitle>
            {{ cd.subtitle }}
          </v-card-subtitle>
        </v-card-item>
        <v-card-actions class="position-absolute top-0 right-0">
          <v-switch @click.stop="setGitProxy" color="primary"></v-switch>
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