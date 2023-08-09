<script setup lang="ts">
import { PropType } from 'vue';
import FileStatus from './components/FileStatus.vue';
import { HashResult } from './types/HashResult'
const props = defineProps({
  hashResult: Object as PropType<HashResult>
})

</script>

<template>
  <FileStatus  />

</template>