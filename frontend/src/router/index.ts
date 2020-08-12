import Vue from "vue";
import VueRouter, { RouteConfig } from "vue-router";
import { getToken } from "@/utils/cookies";
import Main from "@/layouts/Main.vue";
import Login from "@/views/Login.vue";

Vue.use(VueRouter);

const routes: Array<RouteConfig> = [
  {
    path: "/",
    component: Main,
    redirect: "/dashboard",
    children: [
      {
        path: "dashboard",
        component: () =>
          import(/* webpackChunkName: "dashboard" */ "@/views/Home.vue"),
      },
    ],
  },
  {
    path: "/login",
    name: "Login",
    component: Login,
  },
  {
    path: "/about",
    name: "About",
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () =>
      import(/* webpackChunkName: "about" */ "../views/About.vue"),
  },
];

const createRouter = () =>
  new VueRouter({
    // mode: 'history',  // Disabled due to Github Pages doesn't support this, enable this if you need.
    scrollBehavior: (_to, _from, savedPosition) => {
      if (savedPosition) {
        return savedPosition;
      } else {
        return { x: 0, y: 0 };
      }
    },
    base: process.env.BASE_URL,
    routes: routes,
  });

const router = createRouter();

router.beforeEach((to, _from, next) => {
  const isAuthenticated = getToken();
  if (to.name === "Login" && isAuthenticated) next({ name: "Home" });
  else if (to.name !== "Login" && !isAuthenticated) next({ name: "Login" });
  else next();
});

export function resetRouter() {
  const newRouter = createRouter();
  (router as any).matcher = (newRouter as any).matcher; // reset router
}

export default router;
