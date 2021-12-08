import {createRouter, createWebHistory} from 'vue-router'
import Login from '@/views/Login.vue'
import App from "@/views/App.vue";
import Postes from "@/views/app_views/Postes.vue";
import Script from "@/views/app_views/Script.vue";
import Surveillance from "@/views/app_views/Surveillance.vue";
import store from "@/store"

const routes = [
    {
        path: '/',
        name: 'Login',
        component: Login
    },
    {
        path: '/app',
        component: App,
        beforeEnter(to, from, next) {
            if (to.name !== "Login" && !store.getters.isLoggedIn) {
                next({name: "Login"});
            } else {
                next();
            }
        },
        children: [
            {
                path: "postes",
                component: Postes
            },
            {
                path: "script",
                component: Script
            },
            {
                path: "surveillance",
                component: Surveillance
            }
        ]
    }
]

const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
    routes
})

export default router
