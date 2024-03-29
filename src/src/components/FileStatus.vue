<template>
    <div class="p-2 m-2">
        <div @dragenter="startDragAnimation($event)" @dragleave="stopDragAnimation($event)"
            @dragexit="stopDragAnimation($event)">
            <div class="group h-28 bg-gray-50 rounded-md border-2 border-gray-500 border-dashed">
                <div class="pt-4 flex place-content-center">
                    <img v-bind:class="{ 'animate-bounce': hasHover }" class="h-12 w-12" src="../assets/folder.png"
                        alt="Folder">
                </div>
                <div class="flex place-content-center text-xl font-medium text-black"> Drop File Here</div>
            </div>
        </div>
    </div>
    <div class="m-2">
        <label class="block text-left pt-2 text-gray-800 text-sm font-bold mb-1" for="username">
            Target Path
        </label>
        <div class="bg-slate-400 shadow-md rounded px-3 pt-4 pb-1 ">
            <div class="mb-2">
                <o-field>
                    <o-input placeholder="..." :model-value="filePath" type="text" expanded> </o-input>
                </o-field>
                <button @click="openFileDialog()"
                    class="bg-blue-500 rounded-md text-white text-lg font-bold mt-2 w-full">Load
                    File</button>
            </div>
        </div>
        <div class="rounded-xl">
            <p>{{ fileName }}</p>
            <p>{{ filePath }}</p>
        </div>
    </div>
    <div style="height: 250px;" class="m-2">
    <label class="block text-left text-gray-600 font-bold mb-1 pt-2" for="blake2b">Blake2s256</label>
    <div
      class="bg-slate-700 break-words text-white max-h-56 overflow-auto fullshadow-md rounded px-2 pt-1 pb-1  text-sm">
      {{ hash_result?.blake256 }}
    </div>
    <label class="block text-left text-gray-600 font-bold mb-1 pt-2" for="blake2b">Blake2b512</label>
    <div
      class="bg-slate-700 break-words text-white max-h-56 overflow-auto fullshadow-md rounded px-2 pt-1 pb-1  text-sm">
      {{ hash_result?.blake512 }}
    </div>
  </div>
</template>

<script lang="ts" setup>

import { dialog, invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { ref } from 'vue';
import { HashResult } from '../types/HashResult';
import { FileProps } from '../types/FileProps';

let dragTarget: any;
let fileSize = ref(0);
let fileName = ref("");
let hasHover = ref(false);

let filePath = ref("");
let hash_result = ref<HashResult>();

const emits = defineEmits(['fileUpdated'])
emits('fileUpdated', filePath)

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
    dialog.open({ title: "Please select a file...", multiple: false, directory:false }).then((selectedFiles) => {
        if (selectedFiles != null) {
            if (!Array.isArray(selectedFiles)) {
                filePath.value = selectedFiles
                getFileInfo(filePath.value);
            }
        }
    });
}


function getFileInfo(path: string) {
    console.log("Check for file path", path);
    invoke('check_file', { filePath: path }).then((result: any) => {
        let res = result as HashResult;
        console.log("Result", result);
        hash_result.value = {} as HashResult;
        hash_result.value.blake256 = res.blake256;
        hash_result.value.blake512 = res.blake512;
    }).catch((error) => {
        console.log(error);
    })

    invoke('get_file_stats', { filePath: path }).then((result: any) => {
        let res = result as FileProps;
        console.log("Result", res);
    }).catch((error) => {
        console.log(error);
    })
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

function dropFile(event: any) {
    filePath.value = event.payload[0];
    getFileInfo(filePath.value);
}


/**
 * TAURI EVENTS
 */
listen('tauri://file-drop', event => {
    console.log(event);
    hasHover.value = false;
    dropFile(event);
})

listen('tauri://file-drop-cancelled', event => {
    console.log(event);
    hasHover.value = false;
})
</script>