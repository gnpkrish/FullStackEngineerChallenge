<template>
  <div class="home">
    <UserTable :data-set="dataSet" />
  </div>
</template>

<script>
// @ is an alias to /src
import UserTable from "@/components/UserTable.vue";
import { UsersModule } from "@/store/modules/users";

export default {
  name: "Home",
  data() {
    return {
      dataSet: [],
    };
  },
  components: {
    UserTable,
  },
  async created() {
    await UsersModule.ListUsers();
    this.dataSet = UsersModule.users;
  },
};
</script>
