<template>
  <main class="container">
      <h1>Cosmic Clock</h1>
      <br>
      <div v-for="time in times">
        <TimeCard :time="time" v-on:delete="deleteTime"></TimeCard>
        <br>
      </div>
      <div class="row">
        <AddTimeDialog @submit="addTime"></AddTimeDialog>
      </div>
  </main>
</template>

<script setup lang="ts">
import { onMounted, Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import AddTimeDialog from "./components/dialogs/AddTimeDialog.vue";
import TimeCard from "./components/TimeCard.vue";

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

async function deleteTime(time: string) {
  await invoke('delete_time', {
    time: time
  })
  await loadTimes();
}

</script>

<style scoped>
.add-time {
  font-size: 2em;
  padding: .2em .5em;
}
</style>