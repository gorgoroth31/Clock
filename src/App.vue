<template>
  <main class="container">
    <v-tabs color="primary" v-model="tab">
      <v-tab value="alarm">Alarm</v-tab>
      <v-tab value="clock">Clock</v-tab>
    </v-tabs>
    <v-divider></v-divider>
    <br>
    <v-tabs-window v-model="tab">
      <v-tabs-window-item value="alarm">
        <Alarm></Alarm>
      </v-tabs-window-item>

      <v-tabs-window-item value="clock">
        <div>Clock coming soon</div>
      </v-tabs-window-item>
    </v-tabs-window>

    <div>
      {{ data }}
    </div>
  </main>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import Alarm from './components/pages/Alarm.vue';
import { onMounted, Ref, ref } from 'vue'

const tab: Ref<string, string> = ref('alarm')
const data: Ref<string, string> = ref('')

onMounted(async () => {
  data.value = await invoke("is_dark_mode")
})

</script>