import request from "@/utils/request";
import { LoginSubmit } from "@/store/models";

export const login = (data: LoginSubmit) =>
  request({
    url: "/api/user/login",
    method: "post",
    data,
  });

export const generateToken = () =>
  request({
    url: "/graphql",
    method: "post",
    data: {
      query: `
    {
      generateToken {
          bearer
      }
    }
    `,
    },
  });

export const listUsers = () =>
  request({
    url: "/graphql",
    method: "post",
    data: {
      query: `
    {
      users {
          id
          email
          firstName
          lastName
          createdAt
          role
      }
    }
    `,
    },
  });

export const logout = () =>
  request({
    url: "/api/user/logout",
    method: "get",
  });
