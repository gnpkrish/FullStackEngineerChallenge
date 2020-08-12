export interface User {
  email: string;
  token: string;
  username: string;
  bio?: string;
  image?: string;
}

export interface LoginSubmit {
  email: string;
  password: string;
}
