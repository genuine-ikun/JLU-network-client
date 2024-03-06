<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const mesg = ref("");
const name = ref("");
const password = ref("")

let connet = () => {
  let message = name.value
  invoke("change_state", {message: message}).then((response) => mesg.value = String(response))
}

const read_config = () => {
  invoke("js_read_config").then((res) => {
    name.value = res.username
    password.value = res.password
  })
}

onMounted(() => {
  read_config()
})
</script>

<template>
  <form class="input_group" @submit.prevent="connet">
    <input id="username-input" v-model="name" placeholder="用户名" />
    <input id="password-input" type="password" v-model="password" placeholder="密码" />
    <button type="submit">登陆</button>
  </form>
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