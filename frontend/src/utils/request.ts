import axios, { AxiosError } from "axios";
import { message } from "ant-design-vue";
const service = axios.create({
  baseURL: process.env.APP_BASE_API, // url = base url + request url
  timeout: 5000,
  // withCredentials: true // send cookies when cross-domain requests
});

service.interceptors.response.use(
  (response) => {
    return response.data;
  },
  (error) => {
    // console.log("error ", error.response);
    message.error(error.response!.data || "Error");
    return Promise.resolve(error);
  }
);
export default service;
