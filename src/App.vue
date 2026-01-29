<template>
  <v-app>
    <v-main>
      <v-container class="centered-row pa-0">
        <v-tabs bg-color="#1b1b1b" class="main-tabs" color="primary" v-model="tab">
          <v-tab value="alarm">Alarms</v-tab>
          <v-tab value="clock">World Clock</v-tab>
          <v-tab value="stopwatch">Stopwatch</v-tab>
          <v-tab value="timers">Timers</v-tab>
        </v-tabs>
      </v-container>

      <v-tabs-window class="pa-5" v-model="tab">
        <v-tabs-window-item value="alarm">
          <Alarm></Alarm>
        </v-tabs-window-item>

        <v-tabs-window-item value="clock">
          <div>Clock coming soon</div>
        </v-tabs-window-item>

        <v-tabs-window-item value="stopwatch">
          <div>Stopwatch coming soon</div>
        </v-tabs-window-item>

        <v-tabs-window-item value="timers">
          <div>Timer coming soon</div>
        </v-tabs-window-item>
      </v-tabs-window>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import Alarm from './components/pages/Alarm.vue';
import { onMounted, Ref, ref } from 'vue'
import { useTheme } from 'vuetify'

const theme = useTheme()

const tab: Ref<string, string> = ref('alarm')

onMounted(async () => {
  await setDarkMode()
})

async function setDarkMode() {
  let isDarkMode: Boolean = await invoke("is_dark_mode")

  if (isDarkMode) {
    theme.change("customDarkTheme")
  } else {
    theme.change("customLightTheme")
  }
}

</script>

<style scoped>
.main-tabs > div > div > button:first-of-type {
  border-top-left-radius: 20px;
  overflow: hidden;
}
</style>