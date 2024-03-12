<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

const mesg = ref("");
const username = ref("");
const password = ref("")
const run = ref(false)

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

const listen_on_dogcom = async () => {
  await listen<string>('dogcom', (event) => {
    console.log(`Got dogcom: ${event.payload}`)
  } )
}

const change_state = () => {
  run.value = !run.value
  invoke('change_state', {run: run.value})
}

onMounted(() => {
  read_config()
  listen_on_dogcom()
})
</script>

<template>
  <form class="input_group" @submit.prevent="">
    <input id="username-input" v-model="username" placeholder="用户名" />
    <input id="password-input" type="password" v-model="password" placeholder="密码" />
    <button type="submit">登陆</button>

  </form>
  <button @click="write_config">写入</button>
  <input type="text" v-model="run" />
  <button @click="change_state">启动</button>
  <textarea name="" id="" cols="30" rows="10" >{{ mesg }}</textarea>
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
}
</style>