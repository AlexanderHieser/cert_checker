<script setup lang="ts">
import { invoke,dialog } from '@tauri-apps/api'
import { onMounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event'

let hasHover = ref(false);
let dragTarget: any;
let filePath = ref("Test");
let hash_result = ref("");
onMounted(() => {
  invoke('greet', { name: 'World' })
    // `invoke` returns a Promise
    .then((response) => console.log(response))
})

function invokee() {
  console.log("Hello");
  invoke('greet', { name: 'World' }).then((response) => console.log(response))
}
/**
 * Listens to the dropenter event and sets the event target
 * Applies animation
 * @param event DropEnter Event
 */
function startDragAnimation(event: any) {
  dragTarget = event.target;
  event.stopPropagation();
  event.preventDefault();
  console.log("Start Drag animation", event);
  hasHover.value = true;
}

function openFileDialog() {
  dialog.open({title : "Please select a file..." , multiple: false}).then((selectedFiles) => {
    if(selectedFiles != null){
      if(!Array.isArray(selectedFiles)){
        filePath.value = selectedFiles
        getFileInfo(filePath.value);
      }
    }
  });
}

/**
 * Listens to the dropleave event and checks the event target
 * Removes animation
 * @param event DragLeave Event
 */
function stopDragAnimation(event: any) {
  if (dragTarget === event.target) {
    hasHover.value = false;
    event.preventDefault();
    dragTarget = null;
    console.log("Stop Drag animation", event);
    event.stopPropagation();
    event.preventDefault();
  }
}
listen('tauri://file-drop', event => {
  console.log(event);
  hasHover.value = false;
  dropFile(event);
})

function getFileInfo(path:string){
  console.log("Check for file path", path);
  
  invoke('check_file', {filePath: path}).then((result:any) => {
    console.log("Result");
    hash_result.value = result;
  }).catch((error) => {
    console.log(error);
    
  })
}

listen('tauri://file-drop-cancelled', event => {
  console.log(event);
  hasHover.value = false;
})

function dropFile(event:any){
  filePath.value = event.payload[0];
  getFileInfo(filePath.value);
}
</script>

<template>
  <div class="p-2 m-2">
    <div @dragenter="startDragAnimation($event)" @dragleave="stopDragAnimation($event)" @dragexit="stopDragAnimation($event)">
      <div class="group h-28 bg-gray-50 rounded-md border-2 border-gray-500 border-dashed">
        <div class="pt-4 flex place-content-center">
          <img v-bind:class="{ 'animate-bounce': hasHover }" class="h-12 w-12" src="./assets/folder.png" alt="Folder">
        </div>
        <div class="flex place-content-center text-xl font-medium text-black"> Drop File Here</div>
      </div>
    </div>
    <div class="m-full">
      <label class="block text-left pt-2 text-gray-800 text-sm font-bold mb-1" for="username">
        Target Path
      </label>
      <div class="bg-slate-400 shadow-md rounded px-3 pt-4 pb-1 ">
        <div class="mb-2">
          <input
            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="blake" type="text" :value="filePath" placeholder="../../">
          <button @click="openFileDialog()" class="bg-blue-500 rounded-md text-white text-lg font-bold mt-2 w-full">Load
            File</button>
        </div>
      </div>
      <label class="block text-left text-gray-800  font-bold mb-1 pt-2" for="username">
        Result
      </label>
      <div class="bg-slate-300 break-words text-gray-600 max-h-56 overflow-auto fullshadow-md rounded px-3 pt-4 pb-1 mt-2">
        {{ hash_result }}
      </div>
    </div>
  </div>
</template>