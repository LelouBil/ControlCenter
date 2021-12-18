import {ApiClient, LoginApi} from "control_center_api"

export default {
    state: {
        user: null,
    },
    mutations: {
        userLoggedIn(state, payload) {
            state.user = payload.user;
            ApiClient.instance.authentications['JWT'] = {type: "bearer", accessToken: payload.token};
        },
        userLoggedOut(state) {
            state.user = null;
        }
    },
    getters: {
        isLoggedIn(state) {
            return state.user != null;
        }
    },
    actions: {
        async login(context, payload) {
            const formulaire = payload.formulaire;
            const api = new LoginApi();

            try {
                const data = await api.logIn(formulaire);
                const payload2 = {user: formulaire.username, token: data};
                context.commit("userLoggedIn", payload2);
            } catch (error) {
                document.getElementById("pasCo").hidden = false;
            }
        },
        logout(context) {
            context.commit("userLoggedOut");
        }
    },
}