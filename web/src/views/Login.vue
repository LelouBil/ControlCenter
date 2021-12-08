<template>
  <div id="main">
    <img alt="logo" src="@/assets/logo.png">
    <form method="post" action="">
      <h2 class="form-label">Page de connexion</h2>
      <div class="form-floating">
        <input name="username" id="username" type="text" class="form-control" placeholder=" " autocomplete="true"/>
        <label for="username">Username</label>
      </div>
      <div class="form-floating">
        <input name="password" id="password" type="password" class="form-control" placeholder=" " autocomplete="true"/>
        <label for="password">Password</label>
      </div>
      <button type="submit" class="btn btn-primary btn-lg">Connexion</button>
    </form>
  </div>
</template>

<script>

// Pourquoi ?
export default {
  name: 'Login'
}

</script>

<style>
#main {
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
