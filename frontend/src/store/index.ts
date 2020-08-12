import Vue from "vue";
import Vuex from "vuex";
import { IAuthState } from "./modules/auth";
import { IUsersState } from "./modules/users";

Vue.use(Vuex);

export interface IRootState {
  auth: IAuthState;
  user: IUsersState;
}

export default new Vuex.Store<IRootState>({});
