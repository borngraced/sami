import { ActionFunction } from "@remix-run/node";

export const action: ActionFunction = async ({ request }) => {
  let body = await request.formData();
  let slug = body.get("slug");
  let field = body.get("field");
  let method = body.get("method");

  if (!slug || !field || !method) {
    return { error: true, message: "Invalid fields detected" };
  }

  console.log(slug, field, method);

  let token = process.env.TOKEN;
  const headers = {
    headers: {
      Authorization: token!.toString(),
      ContentType: "application/json",
      Method: method!.toString(),
    },
  };
  const res = await fetch(
    `http://127.0.0.1:5500/article/?slug=${slug!.toString()}&field=${field.toString()}`,
    headers
  );
  if (res.statusText === "401") {
    return {
      error: true,
      message: "Problems updating to backdoor",
    };
  }

  console.log(res);
  return "Success";
};
