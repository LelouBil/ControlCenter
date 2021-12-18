<template>
  <div class="main">
    <header class="navbar navbar-expand-lg navbar-dark bg-primary">
      <img alt="logo" class="float-start ms-3" src="@/assets/logo.png">
      <h1 class="d-flex float-start">Control Center</h1>

      <div id="navbarColor01" class="navbar-collapse ms-5">
        <router-link class="navbar-brand" :to="{name: 'Postes'}">Postes</router-link>
        <router-link class="navbar-brand" :to="{name: 'Script'}">Script</router-link>
        <router-link class="navbar-brand" :to="{name: 'Surveillance'}">Surveillance</router-link>
      </div>
      <div class="float-end m-auto">
        <label>Bonjour, <strong class="name">{{ user }}</strong></label><br/>
        <button class="btn btn-secondary" type="button" v-on:click="logout">Déconnexion</button>
        <br>
        <button v-if="spreading" class="btn btn-success" v-on:click="spread">Propagation</button>
        <button v-else class="btn btn-outline-success" v-on:click="spread">Propagation</button>
      </div>
    </header>

    <router-view></router-view>

  </div>
</template>

<script>
export default {
  name: 'App',
  data() {
    return {
      spreading: true
    }
  },
  methods: {
    logout() {
      this.$store.dispatch("logout");
      this.$router.push("/");
    },
    spread() {
      this.spreading = !this.spreading;
    }
  },
  computed: {
    user() {
      return this.$store.state.UserModule.user;
    }
  },
}

</script>

<style scoped>
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

header {
  padding-right: 1rem;
}

.name {
  color: var(--bs-green);
}

button {
  margin-top: .5rem;
  width: 15rem;
  height: 4rem;
  border-radius: 10px;
}

.btn-outline-success:hover {
  background-color: transparent !important;
}
</style>
