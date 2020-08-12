import {
  Module,
  VuexModule,
  getModule,
  Mutation,
  Action,
} from "vuex-module-decorators";
import store from "@/store";
import { listUsers } from "@/api/users";

export interface IUsersState {
  users: string[];
}

@Module({ dynamic: true, store, name: "user", namespaced: true })
class Users extends VuexModule implements IUsersState {
  public users: string[] = [];
  @Mutation
  private SET_USERS(users: string[]) {
    this.users = users;
  }

  @Action({ rawError: true })
  public async ListUsers() {
    const { data } = await listUsers();
    this.SET_USERS(data.users);
  }
}

export const UsersModule = getModule(Users);