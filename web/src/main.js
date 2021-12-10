import {createApp} from 'vue'

import router from './router'

import Main from './Main.vue'
import store from './store'
import {ApiClient, DefaultApi} from 'control_center'

const api_plugin = {
    install(app) {
        ApiClient.instance.basePath = "http://localhost:8000";
        app.config.globalProperties.$api = new DefaultApi();
    }
};

createApp(Main).use(store).use(router).use(api_plugin).mount('#app')
