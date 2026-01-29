import { createApp } from "vue";
import App from "./App.vue";
import '@mdi/font/css/materialdesignicons.css'

// Vuetify
import 'vuetify/styles'
import vuetify from "./plugins/vuetify";

const vuetifyConfig = vuetify

createApp(App).use(vuetifyConfig).mount("#app");
