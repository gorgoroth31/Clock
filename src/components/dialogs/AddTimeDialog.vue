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
          v-on:update:hour="onUpdate"
          v-on:update:minute="onUpdate"
          v-model:model-value=time>
        </v-time-picker>
        <v-card-actions>
          <v-btn
            text="Schließen"
            @click="() => close(isActive)"
          ></v-btn>
          <v-spacer></v-spacer>
          <v-btn
            text="Hinzufügen"
            :disabled="!isValid"
            @click="() => addTime(isActive)">
          </v-btn>
        </v-card-actions>
      </v-card>
    </template>
  </v-dialog>
</template>

<script setup lang="ts">
import { onMounted, ref, Ref } from 'vue';

const emit = defineEmits({
  submit(_time: string) {}
})

const time : Ref<string> = ref("")

const isValid: Ref<boolean> = ref(false)

function close(isActive: Ref<boolean, boolean>) {
  isActive.value = false
  time.value = ""
}

function addTime(isActive: Ref<boolean, boolean>) {
  close(isActive)
  emit('submit', time.value)
}


function onUpdate() {
  isValid.value = time.value !== ""
}
</script>