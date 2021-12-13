import {LoginApi, ApiClient} from "control_center_api"

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

            const data = await api.logIn(formulaire);

            //TODO récupérer infos JWT et chopper username dedans
            const payload2 = {user: formulaire.username, token: data};
            context.commit("userLoggedIn", payload2);
        },
        logout(context) {
            context.commit("userLoggedOut");
        }
    },
}