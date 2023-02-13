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
  <div style="height: 250px;">
    <label class="block text-left text-gray-600 font-bold mb-1 pt-2" for="blake2b">Blake2s256</label>
    <div
      class="bg-slate-700 break-words text-white max-h-56 overflow-auto fullshadow-md rounded px-2 pt-1 pb-1  text-sm">
      {{ hashResult?.blake256 }}
    </div>
    <label class="block text-left text-gray-600 font-bold mb-1 pt-2" for="blake2b">Blake2b512</label>
    <div
      class="bg-slate-700 break-words text-white max-h-56 overflow-auto fullshadow-md rounded px-2 pt-1 pb-1  text-sm">
      {{ hashResult?.blake512 }}
    </div>
  </div>
</template>