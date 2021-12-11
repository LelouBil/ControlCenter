<template>
  <div class="filters">
    <div class="filter">
      <label>On :</label>
      <select class="form-select" v-model="is_on">
        <option>All</option>
        <option>Yes</option>
        <option>No</option>
      </select>
    </div>
    <div class="filter">
      <label>Compromised :</label>
      <select class="form-select" v-model="is_compromised">
        <option>All</option>
        <option>Yes</option>
        <option>No</option>
      </select>
    </div>
    <div class="filter">
      <label>OS :</label>
      <select class="form-select" v-model="os" id="os">
        <option>All</option>
        <option>CentOS</option>
        <option>Windows</option>
        <option>Debian</option>
      </select>
    </div>
    <div class="filter">
      <label for="range">Range :</label>
      <input class="form-control" v-model="range" type="text" id="range" placeholder="150-215">
    </div>
    <button class="btn btn-success" v-on:click="load({is_on, is_compromised, os, range})">Refresh</button>
  </div>
  <h1>Postes</h1>
  <div class="postes">
    <poste v-for="poste in postes" :key="poste.ip" :poste="poste"/>
  </div>
</template>

<script>
import poste from "../../components/poste"

export default {
  name: "Postes",
  data() {
    return {
      postes: [],
      is_on: "All",
      is_compromised: "All",
      os: "All",
      range: "",
    }
  },
  methods: {
    async load(payload) {
      this.postes = await this.$posteApi.listPostes(payload);
    }
  },
  mounted() {
    let payload = {
      is_on: "All",
      is_compromised: "All",
      os: "All",
      range: "",
    }
    this.load(payload);
  },
  components: {
    poste,
  }
}
</script>

<style scoped>
h1 {
  margin: 1rem 1rem 2rem;
}

.postes {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  margin: 0 2rem;
}

.filters {
  display: flex;
  justify-content: center;

  margin: auto;

  background-color: #1a2b3d;;
}

.filter {
  margin: 1rem;
}

button {
  margin: 2.4rem 0 auto 1rem;
}
</style>