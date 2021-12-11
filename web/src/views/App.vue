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
        <label>Bonjour, <strong class="name">{{ user.name }}</strong></label><br/>
        <button class="btn btn-secondary" type="button" v-on:click="logout">Déconnexion</button>

        <div class="form-check m-auto">
          <label class="form-check-label" for="spread">Se répandre</label>
          <input class="form-check-input" type="checkbox" id="spread" v-model="spreading" v-on:change="spread(spreading)">
        </div>
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
    spread(bool) {
      alert("Propagation " + bool);
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
  margin-top: 1rem;
  margin-bottom: .5rem;
  width: 15rem;
  height: 4rem;
  border-radius: 10px;
}

.form-check-input:checked {
  background-color: var(--bs-green);
  border-color: var(--bs-green);
}
</style>
