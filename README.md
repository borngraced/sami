# sami
Sami (Rust, Typescript and PostgresQL Fullstack Web App)

This is a fullstack personal blog application for myself written in: <br/>
```API BACKEND – RUST/actix/postgresQL```<br/>
```FRONTEND — Typescript/Remixjs```


<h2>How To Run Locally</h2>
<ul>
  <li>Register for Auth0 https://auth0.com following the steps here https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/ to get your Bearer token ready</li>
  <li>Add Postgres DB variable to <b>/backdoor/.env</b> format: <b>DB=****************</b></li>
  <li>Add Bearer token variable to <b>/frontdoor/.env</b> format: <b>TOKEN=******</b></li>
  <li>cd into <b>/backdoor</b> and <b>run `cargo run`</b> -- to start the api route locally <b>localhost:5500</b></li>
  <li>cd into <b>/frontdoor</b> and <b>run `npm run dev`</b> -- to start the frontend locally <b>localhost:3000</b></li>
 </ul>
 
 
Boom 🚀🚀🚀!

