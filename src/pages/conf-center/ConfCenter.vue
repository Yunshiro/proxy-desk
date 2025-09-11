<script setup>
import { invoke } from '@tauri-apps/api/core'

const queryConfigs = async () => {
  const result = await invoke('query_proxy_configs')
  console.log(result)
}
</script>

<template>
  <div style="background-color: antiquewhite; height: 100%; width: 100%;">
    <h1>conf</h1>
  </div>
</template>

<style scoped>

</style>