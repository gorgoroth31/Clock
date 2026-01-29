<template>
    <div v-if="times.length === 0">
        <div class="centered-row">No times saved yet</div>
        <br>
    </div>
    <v-container v-else>
        <div v-for="time in times">
            <TimeCard :time="time" v-on:delete="deleteTime"></TimeCard>
            <br>
        </div>
    </v-container>
    <div class="centered-row w-100">
        <AddTimeDialog @submit="addTime"></AddTimeDialog>
    </div>
</template>

<script setup lang="ts">
import { onMounted, Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import AddTimeDialog from "./../dialogs/AddTimeDialog.vue";
import TimeCard from "./../TimeCard.vue";

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
.centered-row {
    display: flex;
    justify-content: center;
}
</style>