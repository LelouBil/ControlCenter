<template>
  <div class="main">Loading</div>
</template>

<script>
export default {
  name: "Postes",
  async mounted() {
    const promise = await this.$api.listPostes();
    console.log(promise);
  }
}
</script>

<style scoped>

</style>