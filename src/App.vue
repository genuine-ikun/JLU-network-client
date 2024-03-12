<script setup lang="ts">
import Input from "./components/Input.vue";
</script>

<template>
  <div class="container w-screen h-screen flex justify-center items-center flex-col pt-8">

    <div class="absolute top-0 w-full h-8" data-tauri-drag-region></div>

    <img src="/user.svg" alt="" class="w-40">
    <!-- <h1 style="margin-bottom: 3em;">JLU网络登录</h1> -->
    <Input />
  </div>
</template>



