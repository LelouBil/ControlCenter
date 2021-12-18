<template>
  <div class="main">
    <div class="left">
      <h3>Change your password</h3>
      <form v-on:submit.prevent="changePass">
        <div class="form-floating">
          <input id="pass" type="password" v-model="pass" autocomplete="false" class="form-control" placeholder=" "/>
          <label for="pass">Enter new password</label>
        </div>

        <div class="form-floating">
          <label for="passConfirm">Confirm new password</label>
          <input id="passConfirm" type="password" v-model="passConfirm" autocomplete="false" class="form-control"
                 placeholder=" "/>
        </div>
        <button type="submit" class="btn btn-primary change">Change</button>
      </form>
    </div>

    <div class="right">
      <h3>All users</h3>
      <div class="user">

      </div>
    </div>
  </div>
</template>

<script>
import {UserPasswordForm, UsersApi} from "control_center_api/src";

export default {
  name: "Users",
  data() {
    return {
      pass: "",
      passConfirm: ""
    }
  },
  methods: {
    async changePass() {
      if (this.pass === this.passConfirm)
        await new UsersApi().changePassword(this.$store.state.UserModule.user, new UserPasswordForm(this.pass));
    }
  }
}
</script>

<style scoped>
.main {
  display: flex;
  justify-content: space-around;
}

.left {
  display: flex;
  flex-direction: column;
  justify-content: center;
  margin-top: 5rem;

  padding: 2rem;

  background-color: var(--bs-gray-800);
  border-radius: 50px;
}

.right {
  display: flex;
  flex-direction: column;
  justify-content: center;
  margin-top: 5rem;

  padding: 2rem;

  background-color: var(--bs-gray-800);
  border-radius: 50px;
}


.form-floating {
  margin: 1rem auto auto;
}

.change {
  width: 10rem;
  height: 3rem;
  margin-top: 1rem;
}

</style>