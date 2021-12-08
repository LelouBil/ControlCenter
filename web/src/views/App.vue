<template>
  <div class="main">
    <header class="navbar navbar-expand-lg navbar-dark bg-primary">
      <img alt="logo" class="float-start ms-3" src="@/assets/logo.png">
      <h1 class="d-flex float-start">Control Center</h1>

      <div id="navbarColor01" class="navbar-collapse ms-5">
        <router-link append class="navbar-brand" to="postes">Postes</router-link>
        <router-link append class="navbar-brand" to="script">Script</router-link>
        <router-link append class="navbar-brand" to="surveillance">Surveillance</router-link>
      </div>
      <div>
        <label>Bonjour, {{ user.name }}</label><br/>
        <button class="btn btn-secondary" type="button" v-on:click="logout">Déconnexion</button>
      </div>
    </header>
    <router-view></router-view>
  </div>
</template>

<script>
// import {mapState} from "vuex";

export default {
  name: 'App',
  methods: {
    logout() {
      this.$store.dispatch("logout");
      this.$router.push("/");
    }
  },
  computed: {
    // ...mapState("store", [
    //   "user"
    // ]),
    user() {
      return this.$store.state.UserModule.user;
    }
  },

}

</script>

<style>
.main {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
}


img {
  width: 100px;
}

h1 {
  font-size: 2rem
}

form {
  margin: 4rem auto auto;
  width: 40rem;
  padding: 1.5rem;

  background-color: var(--bs-gray-800);
  border-radius: 50px;
}

.form-floating {
  width: 20rem !important;
  margin: 1rem auto auto;
}

button {
  margin-top: 2rem;
  width: 15rem;
  height: 4rem;
}
</style>
