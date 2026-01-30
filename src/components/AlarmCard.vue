<template>
    <v-card v-if="props !== undefined">
        <v-card-actions>
            <div class="text-h4">{{ props.alarm?.hour }}:{{ props.alarm?.minute }}</div>
            <v-spacer></v-spacer>        
            <v-btn icon="mdi-delete" @click="handleDelete"></v-btn>
        </v-card-actions>
    </v-card>
</template>

<script setup lang="ts">
import { Alarm } from '../models/alarm';


const props = defineProps<{
    alarm?: Alarm
}>()

const emits = defineEmits({
    delete(_time: Alarm) {}
})

function handleDelete() {
    if (props.alarm === undefined) {
        return
    }
    emits("delete", props.alarm)
}

</script>
