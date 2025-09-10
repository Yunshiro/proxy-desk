<script setup>

const props = defineProps({
  dialog: Boolean
})

const emit = defineEmits(['update:dialog'])

// 使用计算属性确保双向绑定
const isOpen = computed({
  get() {
    return props.dialog
  },
  set(value) {
    emit('update:dialog', value)
  }
})

function close() {
  console.log(isOpen.value)
  isOpen.value = false
}
</script>

<template>
  
</template>

<style scoped></style>