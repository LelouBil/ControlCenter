<template>
  <img alt="Vue logo" src="./assets/logo.png">
  <form method="post" action="">
    <label for="username">Username :</label>
    <input name="username" id="username" type="text"/>
    <label for="password">Password :</label>
    <input name="password" id="password"  type="password"/>
  </form>
</template>

<script>

export default {
  name: 'App'
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
