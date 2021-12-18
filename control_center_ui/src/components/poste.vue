<template>
  <div class="poste" v-on:click="posteCtrl">
    <h4>{{ poste.hostname }}</h4>
    <div class="num">
      <b class="fs-4">{{ poste.os }}</b>
      <br/>
      {{ poste.ip }}
    </div>

    <div v-if="poste.is_on" class="yes">It's <b>ON</b>
      <div v-if="poste.is_compromised" class="yes"><b>COMPROMISED</b></div>
      <div v-else class="no"><b>NOT COMPROMISED</b></div>
    </div>
    <div v-else class="no mt-4">It's <b>OFF</b></div>
  </div>
</template>

<script>
export default {
  name: "poste",
  props: ["poste"],
  methods: {
    posteCtrl() {
      this.$router.push( this.poste.ip);
    }
  }
}
</script>

<style scoped>
.poste {
  background-color: #1a2b3d;

  margin: 3rem;
  width: 15rem;
  height: 16rem;
  padding: 2rem;
  border-radius: 25px;

  cursor: pointer;
}

.num {
  background-color: var(--bs-primary);
  padding: 1.5rem 1rem;
  margin-bottom: .5rem;
  border-radius: 25px;

  font-size: large;
}

.yes {
  color: var(--bs-green)
}

.no {
  color: var(--bs-red)
}

h4 {
  font-weight: bold;
}
</style>