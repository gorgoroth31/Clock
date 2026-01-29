<template>
    <div v-if="alarms.length === 0">
        <div class="centered-row">No times saved yet</div>
        <br>
    </div>
    <v-container v-else>
        <div v-for="time in alarms">
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

const alarms: Ref<string[]> = ref([])

onMounted(() => {
    loadAlarms()
})

async function addTime(time: string) {
    await invoke('add_time', {
        time: time
    })
    await loadAlarms();
}

async function loadAlarms() {
    alarms.value = await invoke("read_alarms")
}

async function deleteTime(time: string) {
    await invoke('delete_time', {
        time: time
    })
    await loadAlarms();
}

</script>
