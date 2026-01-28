<template>
  <v-dialog max-width="500">
    <template v-slot:activator="{ props: activatorProps }">
      <v-btn
        v-bind="activatorProps"
        icon="mdi-plus"
        variant="flat"
      ></v-btn>
    </template>

    <template v-slot:default="{ isActive }">
      <v-card title="Zeit hinzufügen">
        <v-time-picker 
          hide-title
          density="compact"
          format="24hr"
          v-model:model-value=time>
        </v-time-picker>

        <v-card-actions>
          <v-btn
            text="Schließen"
            @click="isActive.value = false"
          ></v-btn>
          <v-spacer></v-spacer>
          <v-btn
            text="Hinzufügen"
            @click="() => addTime(isActive)">

          </v-btn>
        </v-card-actions>
      </v-card>
    </template>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, Ref } from 'vue';

const emit = defineEmits({
  submit(_time: string) {}
})

const time : Ref<string> = ref("")

function addTime(isActive: Ref<boolean, boolean>) {
  isActive.value = false
  emit('submit', time.value)
}
</script>