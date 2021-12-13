import {createApp} from 'vue'

import router from './router'
import Main from './Main.vue'

ApiClient.instance.basePath = "http://localhost:8000";

import store from './store'
import {ApiClient, PostesApi} from 'control_center_api'

const api_plugin = {
    install(app) {
        app.config.globalProperties.$posteApi = new PostesApi();
    }
};

createApp(Main).use(store).use(router).use(api_plugin).mount('#app')
