import {createStore} from 'vuex'
import UserModule from "./user.js"

export default createStore({
    modules: {
        UserModule
    }
})
