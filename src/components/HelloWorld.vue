<script setup lang="ts">
import {invoke} from "@tauri-apps/api";
import {ref} from "vue";
import {ask} from "@tauri-apps/api/dialog";

defineProps<{
  msg: string
}>()

let my_str = ref<string>('init value')

function updateRust() {

  console.log(invoke)
  invoke<string>("greet", {name: "kkkk"}).then(
      (data: string) => {
        console.log(data)
        console.log("type=>" + typeof data)
        my_str.value = data
      }
  )
}

async function dialogAsk() {
  const answer = await ask('This action cannot be reverted. Are you sure?', {
    title: 'Tauri',
    type: 'warning',
  });

  console.log(answer);
}

defineExpose({
  my_str,
  updateRust,
  dialogAsk
})
</script>

<template>
  <div class="greetings">
    <h1 class="green">{{ msg }}</h1>
    <button @click="updateRust">
      the Btn: {{ my_str }}
    </button>

  </div>
</template>

<style scoped>
h1 {
  font-weight: 500;
  font-size: 2.6rem;
  position: relative;
  top: -10px;
}

h3 {
  font-size: 1.2rem;
}

.greetings h1,
.greetings h3 {
  text-align: center;
}

@media (min-width: 1024px) {
  .greetings h1,
  .greetings h3 {
    text-align: left;
  }
}
</style>
