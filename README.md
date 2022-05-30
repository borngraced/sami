# sami
Sami (Rust, Typescript and PostgresQL Fullstack Web App)

This is a fullstack personal blog application for myself written in: <br/>
```API BACKEND – RUST/actix/postgresQL```<br/>
```FRONTEND — Typescript/Remixjs```


<h2>How To Run Locally</h2>
<ul>
  <li>Register for Auth0 https://auth0.com following the steps here https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/ to get your Bearer token ready</li>
  <li>Add Postgres DB variable to `/backdoor/.env;` format: `DB=****************`</li>
  <li>Add Bearer token variable to `/frontdoor/.env;` format: `TOKEN=******`</li>
  <li>cd into `backdoor` and run `cargo run` -- to start the api route locally `localhost:5500`</li>
  <li>cd into `frontdoor`and run `npm run dev` -- to start the frontend locally `localhost:3000`</li>
 </ul>
 
 
Boom 🚀🚀🚀!

