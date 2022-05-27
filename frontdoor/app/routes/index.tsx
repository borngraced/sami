import { json, LoaderFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import Articles from "~/components/common/Articles";
import Header from "~/components/common/Header";

export const loader: LoaderFunction = async () => {
  let token = process.env.TOKEN;
  const headers = {
    headers: {
      Authorization: token!.toString(),
      ContentType: "application/json",
    },
  };
  const res = await fetch("http://127.0.0.1:5500/articles/", headers);
  if (res.statusText === "401") {
    return {
      error: true,
      message: "Problems fetching article from backdoor",
    };
  }
  const body = await res.json();
  return { articles: body };
};

export default function Index() {
  const data = useLoaderData();
  console.log(data);

  return (
    <div className="block h-full w-full">
      <Articles articles={data?.articles?.data} />
    </div>
  );
}
