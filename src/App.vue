<template>
  <main class="container">
    <h1>Welcome to Clockio!</h1>

    <p v-for="time in times">
      {{ time }}
    </p>

    <div class="row">
      <AddTimeDialog @submit="addTime"></AddTimeDialog>
    </div>
  </main>
</template>

<script setup lang="ts">
import { onMounted, Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import AddTimeDialog from "./dialogs/AddTimeDialog.vue";

const times: Ref<string[]> = ref([])

onMounted(() => {
    loadTimes()
})

async function addTime(time: string) {
  await invoke('add_time', {
    time: time
  })
  await loadTimes();
}

async function loadTimes() {
  times.value = await invoke("read_times")
}

</script>

<style scoped>
.add-time {
  font-size: 2em;
  padding: .2em .5em;
}
</style>