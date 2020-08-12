# Employee Review System (ERS)

## Backend

`A Rust authentication server with GraphQL API, Diesel, PostgreSQL session authentication and JWT`

This repository contains boilerplate rust code for getting a GraphQL prototype with JWT up and running quickly.

It uses [actix-web](https://actix.rs/), [Juniper](https://graphql-rust.github.io/juniper/current/),
[Diesel](http://diesel.rs/) and [jsonwebtoken](https://docs.rs/jsonwebtoken)

### Collection of major crates used in ERS

- actix - [link](https://actix.rs/)
- actix-web - [link](https://docs.rs/actix-web/)
- diesel - [link](http://diesel.rs/)
- juniper - [link](https://graphql-rust.github.io/juniper/current/)
- chrono - [link](https://docs.rs/chrono/)
- serde_json - [link](https://docs.serde.rs/serde_json/)
- argon2rs - [link](https://github.com/bryant/argon2rs)
- jsonwebtoken - [link](https://docs.rs/jsonwebtoken)
- anyhow - [link](https://github.com/dtolnay/anyhow)
- thiserror - [link](https://github.com/dtolnay/thiserror)
- shrinkwraprs - [link](https://docs.rs/shrinkwraprs/)

### Required

- [Rustup](https://rustup.rs/)
- Stable Toolchain: `rustup default stable`
- Diesel cli with postgres `cargo install diesel_cli --no-default-features --features "postgres"`
- PostgreSQL database server or use our docker-compose.yml (require docker)

### Getting Started

```sh
cp .env.example .env
docker-compose up
```

### Test the GraphQL API with VScode REST Client

[VScode plugin](https://marketplace.visualstudio.com/items?itemName=humao.rest-client)

See / open TEST.http file in vscode.

### Security

#### Important security considerations

We use session cookies for authentication.

**Why not JWT authentication?**

[Stop Using JWT for sessions and why your solution doesn't work](http://cryto.net/~joepie91/blog/2016/06/19/stop-using-jwt-for-sessions-part-2-why-your-solution-doesnt-work/)

The use of JWT remains secure only if you use adequate storage.

JWT can be use for representing claims to be transferred between two parties.

The private key should only be on this micro-service.
public key can be used on all other parties to decode the token.

#### Generate RSA keys for JWT

In development mode you can keep the one in `/keys` folder.

```shell script
// private key
$ openssl genrsa -out rs256-4096-private.rsa 4096

// public key
$ openssl rsa -in rs256-4096-private.rsa -pubout > rs256-4096-public.pem
```

#### Logging

Logging controlled by middleware::Logger [actix.rs](https://actix.rs/docs/errors/)

To enable debug logging set `RUST_LOG=debug` in `.env`

#### Testing

##### Initialization

First run `yarn` or `npm install` to get all required packages

##### npm run test

To run you can use `npm run test` or `yarn test`.

The testing system designed to automatically build `ers` offline and start in `tests/jest.beforeall.js`
We starting `ers` in order to capture output from both rust and js code using `testci` target

## Frontend

Using Vue eco system to build PWA application.

### Collection of major packags used to build UI
- Vue - [link](https://vuejs.org)
- Vuex - [link](https://vuex.vuejs.org/)
- Vue Router - [link](https://router.vuejs.org/)
- antd Vue - [link](https://www.antdv.com/docs/vue/introduce/)
- Vue I18n - [link](https://kazupon.github.io/vue-i18n/)

## Start Services

```shell
$ docker-compose -f docker-compose.local.yml up --build
```

The app running on :8000 port

In browser http://localhost:8000/