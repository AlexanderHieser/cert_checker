<script setup lang="ts">
</script>

<template>
  <div class="p-2 m-2">
    <div class="content-center h-28 bg-gray-50 rounded-md border-solid border-2 border-gray-500 border-dashed">
      <p class="text-center pt-11 font-bold text-slate-400">Drop File Here</p>
    </div>
    <div class="m-full">
      <label class="block text-left pt-2 text-gray-800 text-sm font-bold mb-1" for="username">
        Target Path
      </label>
      <div class="bg-slate-400 shadow-md rounded px-3 pt-4 pb-1 ">
        <div class="mb-2">
          <input
            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="blake" type="text" placeholder="../../">
          <button class="bg-blue-500 rounded-md text-white text-lg font-bold mt-2 w-full">Load File</button>
        </div>
      </div>
      <label class="block text-left text-gray-800  font-bold mb-1 pt-2" for="username">
        Result
      </label>
      <div
        class="bg-slate-300 text-gray-600 max-h-56 overflow-auto text-white pb-4 h-fullshadow-md rounded px-3 pt-4 pb-1 mt-2">
        Lorem ipsum dolor, sit amet consectetur adipisicing elit. Sunt harum quis placeat rem, ratione, voluptates nisi
        deleniti mollitia voluptatem accusantium aut dolor nesciunt dolorem. Asperiores minus cum pariatur fugit
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Asperiores voluptate maiores officiis impedit nemo
        voluptates repudiandae assumenda optio, ipsam ipsa, necessitatibus possimus nobis. Maiores quia, ex earum
        nesciunt neque magni.
        Lorem ipsum dolor sit, amet consectetur adipisicing elit. Minima nisi in enim eligendi molestias unde.
        Blanditiis incidunt tenetur enim eaque nisi sed harum amet? Aperiam maiores tempora quas quibusdam placeat!
        eligendi!
      </div>
    </div>
  </div>
</template>