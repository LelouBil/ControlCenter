import {createStore} from 'vuex'
import UserModule from "./user.js"
import VuexPersist from 'vuex-persist'

const vuexLocalStorage = new VuexPersist({
    storage: window.sessionStorage,
})

export default createStore({
    modules: {
        UserModule,
    },
    plugins: [vuexLocalStorage.plugin]
})
