import {
  Module,
  VuexModule,
  getModule,
  Mutation,
  Action,
} from "vuex-module-decorators";
import store from "@/store";
import { login, logout, generateToken } from "@/api/users";
import { getToken, setToken, removeToken } from "@/utils/cookies";
// import { resetRouter } from "@/router";

export interface IAuthState {
  token: string;
  name: string;
  avatar: string;
  introduction: string;
  roles: string[];
  email: string;
}

@Module({ dynamic: true, store, name: "auth", namespaced: true })
class Auth extends VuexModule implements IAuthState {
  public token = getToken() || "";
  public name = "";
  public avatar = "";
  public introduction = "";
  public roles: string[] = [];
  public email = "";

  @Mutation
  private SET_TOKEN(token: string) {
    this.token = token;
  }

  @Action({ rawError: true })
  public async Login(loginInfo: { email: string; password: string }) {
    let email = loginInfo.email;
    const password = loginInfo.password;
    email = email.trim();
    await login({ email, password });
  }

  @Action({ rawError: true })
  public async GenerateToken() {
    const { data } = await generateToken();
    const token = data.generateToken.bearer;
    setToken(token);
    this.SET_TOKEN(token);
  }

  @Action({ rawError: true })
  public async Logout() {
    if (this.token === "") {
      throw Error("LogOut: token is undefined!");
    }
    await logout();
    removeToken();
    // resetRouter();

    // Reset visited views and cached views
    this.SET_TOKEN("");
  }
}

export const AuthModule = getModule(Auth);
