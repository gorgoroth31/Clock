<template>
    <div v-if="alarms.length === 0">
        <div class="centered-row">No times saved yet</div>
        <br>
    </div>
    <v-container v-else>
        <div v-for="alarm in alarms">
            <AlarmCard :alarm="alarm" v-on:delete="deleteTime"></AlarmCard>
            <br>
        </div>
    </v-container>
    <div class="centered-row w-100">
        <AddTimeDialog @submit="addAlarm"></AddTimeDialog>
    </div>
</template>

<script setup lang="ts">
import { onMounted, Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import AddTimeDialog from "./../dialogs/AddTimeDialog.vue";
import { Alarm } from "../../models/alarm";
import AlarmCard from "../AlarmCard.vue";

const alarms: Ref<Alarm[]> = ref([])

onMounted(() => {
    loadAlarms()
})

async function addAlarm(alarm: string) {
    await invoke('add_alarm', {
        alarm: alarm
    })
    await loadAlarms();
}

async function loadAlarms() {
    alarms.value = await invoke("read_alarms")
}

async function deleteTime(alarm: Alarm) {
    await invoke('delete_alarm', {
        alarm: alarm
    })
    await loadAlarms();
}

</script>
