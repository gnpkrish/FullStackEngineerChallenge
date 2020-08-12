<template>
  <a-table
    :columns="columns"
    :row-key="(record) => record.id"
    :data-source="dataSet"
  >
  </a-table>
</template>
<script lang="ts">
import { Component, Vue, Prop } from "vue-property-decorator";
const columns = [
  {
    title: "Email",
    dataIndex: "email",
  },
  {
    title: "Firstname",
    dataIndex: "firstName",
  },
  {
    title: "Lastname",
    dataIndex: "lastName",
  },
  {
    title: "Role",
    dataIndex: "role",
  },
  {
    title: "CreatedAt",
    dataIndex: "createdAt",
  },
  {
    title: "operation",
    dataIndex: "operation",
    scopedSlots: { customRender: "operation" },
  },
];
@Component
export default class UserTable extends Vue {
  columns = columns;
  @Prop({ default: [{}] })
  dataSet!: string[];
}
</script>
