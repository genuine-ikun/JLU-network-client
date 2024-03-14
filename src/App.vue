<script setup lang="ts">
import Input from "./components/Input.vue";
</script>

<template>
  <div class="container w-screen h-screen flex  items-center flex-col pt-2">

    <div class="absolute top-0 w-full h-10" data-tauri-drag-region></div>

    <img src="/user.svg" alt="" class="w-40 h-1/2">
    <h1 class="text-2xl font-semibold mb-12">JLU网络登录</h1>
    <Input />
  </div>
</template>



