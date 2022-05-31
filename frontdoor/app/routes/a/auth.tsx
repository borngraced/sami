import { ActionFunction, json } from "@remix-run/node";
import { useActionData } from "@remix-run/react";
import { ReactNode } from "react";

export const action: ActionFunction = async ({ request }) => {
  let body = await request.formData();
  let email = body.get("email");
  let password = body.get("password");

  if (!email || !password) {
    return { error: true, message: "Invalid / missing field detected" };
  }

  let token = process.env.TOKEN;
  const res = await fetch(`http://127.0.0.1:5500/auth/`, {
    headers: {
      Authorization: token!.toString(),
      ContentType: "application/json",
      Method: "POST",
    },
    body: body,
  });
  console.log("rr", res.body);
  if (res.statusText != "200") {
    return {
      error: true,
      message: "Problems updating to backdoor",
    };
  }
  let resp = res.body;
  return resp;
};

const AuthPage = ({}) => {
  const data = useActionData();
  console.log("ffF", data);

  return (
    <div className="block max-w-md mx-auto p-2 md:p-4 my-40">
      <div className="block w-full title text-center text-xl font-mono antialiased  text-gray-200 my-6">
        Sami Authorize
      </div>
      <form method="POST">
        <label htmlFor="Email" className="block w-full h-full email mb-4">
          <input
            type="email"
            name="email"
            className="block w-full h-12 bg-slate-700 rounded-md
           shadow-md placeholder:text-slate-400 text-gray-300 font-mono antialiased text-md px-4 outline-none"
            placeholder="My email"
          />
        </label>
        <label htmlFor="Password" className="block w-full h-full password mb-4">
          <input
            type="password"
            name="password"
            className="block w-full h-12 bg-slate-700 rounded-md
           shadow-md placeholder:text-slate-400 text-gray-300 font-mono antialiased text-md px-4 outline-none"
            placeholder="My password"
          />
        </label>
        <button
          type="submit"
          className="flex justify-center items-center w-full h-12 bg-black bg-opacity-60 text-slate-400 hover:text-slate-200 hover:bg-opacity-80 rounded-md
           shadow-lg"
        >
          Authorize Me
        </button>
      </form>
    </div>
  );
};

export default AuthPage;
