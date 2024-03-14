<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { UnlistenFn, listen } from "@tauri-apps/api/event";

const mesg = ref("");
const run = ref(false)
const username = ref("");
const password = ref("")

const read_config = () => {
  invoke("js_read_config").then((res: any) => {
    username.value = res.username
    password.value = res.password
  })
}

const write_config = () => {
  invoke("js_write_config", {username: username.value,
    password: password.value,
    hostIp: "sdfd",
    mac: "0xfffff"
  }).then((res) => console.log(res))
}

const change_state = () => {
  invoke("change_state", {run: run.value})
}

let unlisten: UnlistenFn;

onMounted(async () => {
  read_config()
  unlisten = await listen<string>('dogcom', (event) => {
    console.log(`Got dogcom: ${event.payload}`)
  })
})

onUnmounted(() => {
  unlisten()
})
</script>

<template>
  <form class="input_group text-md">
    <input v-model="username" placeholder="用户名" class="font-sans" />
    <input type="password" v-model="password" placeholder="密码" class="font-sans"/>
    <button class="bg-gray-200 hover:bg-gray-300 px-8 py-3 rounded-lg mt-3" @click="change_state();write_config()">登 录</button>
  </form>
  <!-- <textarea name="" id="" cols="30" rows="10" >{{ mesg }}</textarea> -->
</template>

<style scoped>
.input_group{
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  width: 80%;
}

input{
  margin-bottom: 1em;
  border-radius: 10px;
  border: 2px solid #8c8c8c;
  outline: none;
  padding: 3px 3px 3px 10px;
}
</style>