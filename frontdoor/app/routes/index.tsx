import { json, LoaderFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";

export const loader: LoaderFunction = async () => {
  let token =
    "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImVIUEpUNEdDb3B6ams5b3M3dldJSCJ9.eyJpc3MiOiJodHRwczovL2Jsb2dneS51cy5hdXRoMC5jb20vIiwic3ViIjoiSlBuU1h6V0RDSjh1NDB6alREOW83eUszeGdMR01ORkdAY2xpZW50cyIsImF1ZCI6ImJzdG8wNDAzIiwiaWF0IjoxNjUzMTY0OTQyLCJleHAiOjE2NTMyNTEzNDIsImF6cCI6IkpQblNYeldEQ0o4dTQwempURDlvN3lLM3hnTEdNTkZHIiwiZ3R5IjoiY2xpZW50LWNyZWRlbnRpYWxzIn0.MCIadWu0HfNoEhzsGfzfMJVehex6QJu7g_feJLNYScVhtKbIMbiTpjO2OoWTqSfvwaPvuHQ1Xyxq-xykzLVhvzck_JZZE208wLWeGaqfBnbhvkjfO2I6n6c-_5UkyFFhudIRyXHczRJMiFYhzgbF57Zv4WyAG3D8fdyTFgNw09nCy1acuMD2XZVHIYzJd_X0iO14FDL7bY6bxX8ubiu2CZHQX7wNoJUkq0YEb2P76SqSa6As5HZUzzcpAWuGfhd-REM07MYiibPxnun8ZunxwThtUuV51l4P_O4NFoOSC61CzAKKNSy4iS8hBDj7pmNvLidWesOZHayTln-UWsWD2g";

  const headers = {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  };
  const res = await fetch("http://127.0.0.1:5050/user/2", headers);
  const r = json(await res.json());
  console.log(r);
  return r;
};

export default function Index() {
  const data = useLoaderData();
  console.log(data);

  return (
    <div style={{ fontFamily: "system-ui, sans-serif", lineHeight: "1.4" }}>
      <h1>Welcome to Remix</h1>
      <ul>
        <li>
          <a
            target="_blank"
            href="https://remix.run/tutorials/blog"
            rel="noreferrer"
          >
            15m Quickstart Blog Tutorial
          </a>
        </li>
        <li>
          <a
            target="_blank"
            href="https://remix.run/tutorials/jokes"
            rel="noreferrer"
          >
            Deep Dive Jokes App Tutorial
          </a>
        </li>
        <li>
          <a target="_blank" href="https://remix.run/docs" rel="noreferrer">
            Remix Docs
          </a>
        </li>
      </ul>
    </div>
  );
}
