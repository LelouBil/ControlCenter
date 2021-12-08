export default {
    state: {
        user: null
    },
    mutations: {
        userLoggedIn(state, payload) {
            state.user = payload.user;
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
        login(context, payload) {
            const user = {name: payload.username};
            context.commit("userLoggedIn", {user});
        },
        logout(context) {
            context.commit("userLoggedOut");
        }
    },
}