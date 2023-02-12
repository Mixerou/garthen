<script setup>
import Avatar from 'vue-boring-avatars'

defineProps({
  username: {
    type: String,
    required: true,
  },
})

// TODO: Choose the perfect colours
const colors = ['#E6626F', '#EFAE78', '#F5E19C', '#A2CA8E', '#66AF91']
// const colors = ['#FFBD87', '#FFD791', '#F7E8A6', '#D9E8AE', '#BFE3C0']
// const colors = ['#95AA61', '#121310', '#F6F8EE', '#D6E68A', '#899752']
</script>

<template>
  <Avatar variant="beam" :colors="colors" :name="username" square />
</template>

<style lang="scss" scoped></style>
