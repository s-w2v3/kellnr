<template>
  <h2 class="k-h2">Users</h2>
  <template v-for="item in items" :key="item">
    <div class="userMgmt glass">
      <span class="userName">{{ item.name }}</span>
      <span class="" v-if="item.is_admin">Role: admin</span>
      <span class="" v-else>Role: user</span>
      <span class="tag is-info is-light" v-if="item.is_read_only">
        <a @click="set_read_only(item.name, false, item)">Remove Read-only</a>
      </span>
      <span class="tag is-primary is-light" v-else>
        <a @click="set_read_only(item.name, true, item)">Make Read-only</a>
      </span>
      <span class="tag is-warning is-light resetPwd">
        <a @click="resetPwd(item.name)">Reset password</a>
      </span>
      <span class="tag is-danger is-light">
        <a @click="deleteUser(item.name)">Delete</a>
      </span>
    </div>
  </template>

  <status-notification :status="changeUserStatus" @update:clear="changeUserStatus = $event">
    {{ changeUserMsg }}
  </status-notification>

  <h3 class="k-h3">Add User</h3>
  <form>
    <div class="field">
      <div class="control is-expanded has-icons-left">
        <input
            class="input is-info"
            v-model="name"
            placeholder="Name"
            type="text"
        />
        <span class="icon is-small is-left">
          <i class="fas fa-user"></i>
        </span>
      </div>
    </div>
    <div class="field">
      <div class="control has-icons-left">
        <input
            class="input is-info"
            v-model="pwd1"
            placeholder="Password"
            type="password"
        />
        <span class="icon is-small is-left">
          <i class="fas fa-lock"></i>
        </span>
      </div>
    </div>
    <div class="field">
      <div class="control has-icons-left">
        <input
            class="input is-info"
            v-model="pwd2"
            placeholder="Confirm Password"
            type="password"
        />
        <span class="icon is-small is-left">
          <i class="fas fa-lock"></i>
        </span>
      </div>
    </div>
    <div class="field">
      <label class="checkbox">
        <input v-model="is_admin" type="checkbox"/> Is Admin
      </label>
    </div>
    <div class="field">
      <label class="checkbox">
        <input v-model="is_read_only" type="checkbox"> Is Read-only
      </label>
    </div>

    <status-notification :status="addUserStatus" @update:clear="addUserStatus = $event">
      {{ addUserMsg }}
    </status-notification>

    <div class="control">
      <button class="button is-info" @click.prevent="addUser">Add</button>
    </div>
  </form>
</template>

<script setup lang="ts">
import StatusNotification from "./StatusNotification.vue";
import {onBeforeMount, ref} from 'vue'
import {ADD_USER, DELETE_USER, LIST_USERS, RESET_PWD, USER_READ_ONLY} from "../remote-routes";
import axios from "axios";
import {useRouter} from "vue-router";

const router = useRouter();
const addUserStatus = ref("")
const addUserMsg = ref("")
const changeUserStatus = ref("")
const changeUserMsg = ref("")
const items = ref([])
const name = ref("")
const pwd1 = ref("")
const pwd2 = ref("")
const is_admin = ref(false)
const is_read_only = ref(false)

onBeforeMount(() => {
  getUsers()
})

function addUser() {
  const postData = {
    name: name.value,
    pwd1: pwd1.value,
    pwd2: pwd2.value,
    is_admin: is_admin.value,
    is_read_only: is_read_only.value
  };

  axios
      .post(ADD_USER, postData)
      .then((res) => {
        if (res.status == 200) {
          addUserStatus.value = "Success";
          addUserMsg.value = "User successfully added.";
          // Update user list
          getUsers();
        }
      })
      .catch((error) => {
        if (error.response) {
          addUserStatus.value = "Error";
          addUserMsg.value = "User could not be added.";

          if (error.response.status == 404) {
            // "Unauthorized. Login first."
            router.push("/login");
          } else if (error.response.status == 400) {
            addUserMsg.value = "Password do not match";
          } else if (error.response.status == 500) {
            addUserMsg.value = "User could not be added";
          } else {
            addUserMsg.value = "Unknown error";
          }
        }
      });
}

function getUsers() {
  axios
      .get(LIST_USERS, {cache: false})
      .then((res) => {
        if (res.status == 200) {
          items.value = res.data;
        }
      })
      .catch((error) => {
        console.log(error);
      });
}

function deleteUser(name: string) {
  if (confirm('Delete user "' + name + '"?')) {
    axios
        .delete(DELETE_USER(name))
        .then((res) => {
          if (res.status == 200) {
            changeUserStatus.value = "Success";
            changeUserMsg.value = 'User "' + name + '" deleted';
            getUsers();
          }
        })
        .catch((error) => {
          changeUserStatus.value = "Error";
          if (error.response.status == 404) {
            // "Unauthorized. Login first."
            router.push("/login");
          } else if (error.response.status == 500) {
            changeUserMsg.value = "User could not be deleted";
          } else {
            changeUserMsg.value = "Unknown error";
          }
        });
  }
}

function resetPwd(name: string) {
  if (confirm('Reset password for "' + name + '"?')) {
    axios
        .post(RESET_PWD(name))
        .then((res) => {
          if (res.status == 200) {
            changeUserStatus.value = "Success";
            changeUserMsg.value =
                'Password for "' +
                name +
                '" reset to "' +
                res.data["new_pwd"] +
                '".\nNotify the user to change the password on the next login.';
          }
        })
        .catch((error) => {
          changeUserStatus.value = "Error";
          if (error.response.status == 404) {
            // "Unauthorized. Login first."
            router.push("/login");
          } else if (error.response.status == 500) {
            changeUserMsg.value = "Password could not be reset";
          } else {
            changeUserMsg.value = "Unknown error";
          }
        });
  }
}

function set_read_only(name: string, state: boolean, item: object) {
  if (!confirm(state ? 'Make "' + name + '" read-only' : 'Remove read-only from "' + name + '"')) {
    return;
  }

  axios
    .post(USER_READ_ONLY(name), {state: state})
    .then((res) => {
      if (res.status == 200) {
        changeUserStatus.value = "Success";
        if (state) {
          changeUserMsg.value = '"' + name + '" was made read-only "';
          item.is_read_only = true; // update UI
        } else {
          changeUserMsg.value = 'Removed read-only from "' + name + '"';
          item.is_read_only = false; // update UI
        }
      }
    })
    .catch((error) => {
      changeUserStatus.value = "Error";
      if (error.response.status == 404) {
        // "Unauthorized. Login first."
        router.push("/login");
      } else if (error.response.status == 500) {
        changeUserMsg.value = "Read-only could not be modified";
      } else {
        changeUserMsg.value = "Unknown error";
      }
    });
}
</script>

<style scoped>
.userMgmt {
  border-radius: 2px;
  margin: 0.5rem 0 0.5rem 0;
  padding: 0.5rem;
  display: grid;
  grid-template-columns: 1fr 1fr max-content max-content max-content;
}

.userName {
  font-weight: bolder;
}

.resetPwd {
  margin-right: 0.3rem;
}
</style>
