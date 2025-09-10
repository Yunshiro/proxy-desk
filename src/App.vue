<script setup>

</script>

<template>
  <v-layout id="app-layout">
    <v-app-bar id="app-bar" title="Application bar"></v-app-bar>
    <v-navigation-drawer width="180" permanent>
      <v-list nav>
        <v-list-item title="代理" to="/" link></v-list-item>
        <v-list-item title="配置" to="/confcenter"></v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main style="height: 100vh; overflow: hidden;">
      <RouterView />
    </v-main>
  </v-layout>
</template>

<style scoped>
#app-layout {
  height: 100vh;
}

#app-bar {
  height: 60px;
}


</style>
