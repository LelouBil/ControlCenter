import {createRouter, createWebHistory} from 'vue-router'
import Login from '../views/Login.vue'
import App from "../views/App.vue";
import Postes from "../views/app_views/Postes";
import Script from "../views/app_views/Script.vue";
import Surveillance from "../views/app_views/Surveillance.vue";
import store from "../store";
import PosteCtrl from "../views/PosteCtrl";

const routes = [
    {
        path: '/',
        name: 'Login',
        component: Login
    },
    {
        path: '/app',
        name: 'App',
        component: App,
        beforeEnter(to, from, next) {
            if (to.name !== "Login" && !store.getters.isLoggedIn) {
                next({name: "Login"});
            } else if (to.name === "App") {
                next({name: "Postes"});
            } else {
                next();
            }
        },
        children: [
            {
                path: "postes",
                name: "Postes",
                component: Postes
            },
            {
                path: "script",
                name: "Script",
                component: Script
            },
            {
                path: "surveillance",
                name: "Surveillance",
                component: Surveillance
            },
            {
                path: ":ip",
                name: "PosteCtrl",
                component: PosteCtrl
            }
        ],
    }
]

const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
    routes
})

export default router
