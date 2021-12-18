<template>
  <div class="main">
    <img alt="logo" src="@/assets/logo.png">
    <form v-on:submit.prevent="login">
      <h2 class="form-label">Page de connexion</h2>
      <div class="form-floating">
        <input id="username" v-model="formulaire.username" autocomplete="true" class="form-control" placeholder=" " type="text"
               required/>
        <label for="username">Username</label>
      </div>
      <div class="form-floating">
        <input id="password" v-model="formulaire.password" autocomplete="true" class="form-control" placeholder=" " type="password"
               required/>
        <label for="password">Password</label>
      </div>
      <button class="btn btn-primary btn-lg" type="submit">Connexion</button>
    </form>
  </div>
</template>

<script>
import {LoginForm} from "control_center_api";

export default {
  name: 'Login',
  data() {
    return {
      formulaire : new LoginForm()
    }
  },
  methods: {
    async login() {
      await this.$store.dispatch("login", {formulaire: this.formulaire});
      await this.$router.replace({name: "App"});
    }
  }
}

</script>

<style scoped>
.main {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  margin-top: 3rem;
}

img {
  width: 200px;
}

h2 {
  margin-bottom: 2rem !important;
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
