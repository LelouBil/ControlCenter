<template>
  <div class="filters">
    <div class="filter">
      <label>On :</label>
      <select class="form-select btn-info" v-model="filter.is_on">
        <option v-bind:value=null>All</option>
        <option v-bind:value=true>Yes</option>
        <option v-bind:value=false>No</option>
      </select>
    </div>
    <div class="filter">
      <label>Compromised :</label>
      <select class="form-select btn-info" v-model="filter.is_compromised">
        <option v-bind:value=null>All</option>
        <option v-bind:value=true>Yes</option>
        <option v-bind:value=false>No</option>
      </select>
    </div>
    <div class="filter">
      <label>OS :</label>
      <select class="form-select btn-info" v-model="filter.os" id="os">
        <option>All</option>
        <option>CentOS</option>
        <option>Windows</option>
        <option>Debian</option>
      </select>
    </div>
    <div class="filter">
      <label for="range">Range :</label>
      <div class="range">
        <input class="form-control btn-info" v-model.lazy="filter.range[0]" type="number" id="range" placeholder="From">
        <input class="form-control btn-info" v-model.lazy="filter.range[1]" type="number" id="range2" placeholder="To">
      </div>
    </div>
    <button class="btn btn-success" v-on:click="load(filter)">Refresh</button>
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
      filter: {
        is_on: null,
        is_compromised: null,
        os: "All",
        range: [],
      }
    }
  },
  methods: {
    async load(filter) {
      this.postes = await this.$posteApi.listPostes(filter);
    }
  },
  mounted() {
    this.load(this.filter);
  },
  components: {
    poste,
  }
}
</script>

<style scoped>
h1 {
  margin: 1rem 1rem .5rem;
}

.postes {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
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
  margin: 2.5rem 0 auto .5rem;
}

.range {
  display: flex;
}

.range input {
  width: 5rem;
}

.range input:first-child {
  margin-right: .5rem;
}

.range input::placeholder {
  color: white;
}

input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

/* Firefox */
input[type=number] {
  -moz-appearance: textfield;
}
</style>