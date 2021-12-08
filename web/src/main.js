import {createApp} from 'vue'

import router from './router'

import Main from './Main.vue'
import store from './store'

createApp(Main).use(store).use(router).mount('#app')
