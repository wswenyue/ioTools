<script setup lang="ts">
import {invoke} from "@tauri-apps/api";
import {ref} from "vue";

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

defineExpose({
  my_str,
  updateRust
})
</script>

<template>
  <div class="greetings">
    <h1 class="green">{{ msg }}</h1>
    <button @click="updateRust">
      the Btn: {{ my_str }}
    </button>
    <h3>
      You’ve successfully created a project with
      <a href="https://vitejs.dev/" target="_blank" rel="noopener">Vite</a> +
      <a href="https://vuejs.org/" target="_blank" rel="noopener">Vue 3</a>. What's next?
    </h3>
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
