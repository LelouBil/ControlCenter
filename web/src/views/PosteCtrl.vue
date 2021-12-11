<template>
  <div class="main">
    <h1>{{ $route.params.ip }}</h1>
    <div class="content">
      <div class="shared">
        <div class="screen">Écran</div>
        <div class="console">Console</div>
      </div>
      <div>
        <input id="souris" type="checkbox" v-model="souris">
        <label class="ms-1 mt-3" for="souris">Souris écran</label>
      </div>
    </div>
    <button type="button" class="btn btn-danger" v-on:click="abandon">Abandonner le poste</button>
  </div>
</template>

<script>
export default {
  name: "PosteCtrl",
  data() {
    return {
      souris: "false",
    }
  },
  methods: {
    abandon() {
      if (confirm("Voulez-vous vraiment abandonner ce poste ?")) {
        if (confirm("Est-ce votre dernier mot ?")) {
          alert('TODO lol')
        }
      }
    }
  }
}
</script>

<style scoped>
.main {
  display: flex;
  flex-direction: column;
}

h1 {
  margin: 1rem 1rem .7rem;
}

.content {
  margin: auto auto 1rem;
  padding: 3rem 3rem 1rem;
  border-radius: 50px;
  background-color: #1a2b3d;
}

.shared {
  width: 100rem;
  height: 40rem;

  display: flex;
  justify-content: space-between;
}

.screen {
  width: 60rem;
}

.console {
  width: 30rem;
}

.shared div {
  background-color: var(--bs-primary);
  border-radius: 20px;
}

button {
  width: 12rem;
  margin: auto auto 1rem;

  font-weight: 550;
}
</style>