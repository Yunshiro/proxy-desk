<script setup>
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'

const confs = ref([])

const queryConfigs = async () => {
  const result = await invoke('query_proxy_configs')
  confs.value = result
  console.log(result)
}

const addConfig = () => {

}

onMounted(() => {
  queryConfigs()
})


</script>


<template>
  <div id="config-table">
    <div id="tools-kit">
      <v-btn color="primary" @click="addConfig">Add</v-btn>
    </div>
    <v-table>
      <thead>
        <tr>
          <th class="text-left">
            title
          </th>
          <th class="text-left">
            system_type
          </th>
          <th class="text-left">
            proxy_type
          </th>
          <th class="text-left">
            proxy_url
          </th>
          <th class="text-left">
            local_env_path
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in confs" :key="item.id">
          <td>{{ item.title }}</td>
          <td>{{ item.system_type }}</td>
          <td>{{ item.proxy_type }}</td>
          <td>{{ item.proxy_url }}</td>
          <td>{{ item.local_env_path }}</td>
        </tr>
      </tbody>
    </v-table>
  </div>
</template>

<style scoped></style>