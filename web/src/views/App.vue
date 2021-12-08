<template>
  <div class="main">
    <nav class="navbar navbar-expand-lg navbar-dark bg-primary">
      <img class="float-start ms-3" alt="logo" src="@/assets/logo.png">
      <h1 class="d-flex float-start">Control Center</h1>

      <div class="navbar-collapse ms-5" id="navbarColor01">
        <router-link class="navbar-brand" to="@/app_views/postes">Postes</router-link>
        <router-link class="navbar-brand" to="script">Script</router-link>
        <router-link class="navbar-brand" to="surveillance">Surveillance</router-link>
      </div>
    </nav>
    <router-view/>
  </div>
</template>

<script>

// Pourquoi ?
export default {
  name: 'App'
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
