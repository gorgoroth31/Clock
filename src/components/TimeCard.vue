<template>
    <v-card v-if="props !== undefined">
        <v-card-actions>
            <div class="text-h4">{{ props.time }}</div>
            <v-spacer></v-spacer>        
            <v-btn icon="mdi-delete" @click="handleDelete"></v-btn>
        </v-card-actions>
    </v-card>
</template>

<script setup lang="ts">

const props = defineProps<{
    time?: string
}>()

const emits = defineEmits({
    delete(_time: string) {}
})

function handleDelete() {
    if (props.time === undefined) {
        return
    }
    emits("delete", props.time)
}

</script>
