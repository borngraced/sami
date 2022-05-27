import { json, LoaderFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import React from "react";
import { ArticleIT } from "~/components/common/Articles";

export const loader: LoaderFunction = async ({ params }) => {
  let token = process.env.TOKEN;
  const headers = {
    headers: {
      Authorization: token!.toString(),
      ContentType: "application/json",
    },
  };
  const res = await fetch(
    `http://127.0.0.1:5500/article/${params.article}/`,
    headers
  );
  if (res.statusText === "401") {
    return {
      error: true,
      message: "Problems fetching article from backdoor",
    };
  }
  const article = await res.json();
  return article;
};

type Props = {};

const ArticleSlug = (props: Props) => {
  const article = useLoaderData();
  console.log(article);
  let show_tags = article?.data.tags?.map((e: String) => {
    return <span className="tags text-sm text-gray-400">{"#" + e + " "}</span>;
  });
  return (
    <main className="mx-auto max-w-4xl my-10">
      <h1 className="mb-6 text-black text-center text-2xl">
        {article?.data?.title}
      </h1>
      <div
        className="post-body text-xl"
        dangerouslySetInnerHTML={{ __html: article.data.content }}
      />
      <div className="block">{show_tags}</div>

      <div className="like_post block text-sm text-gray-400">
        Article likes 👍 {article?.data.likes}
      </div>
    </main>
  );
};
export default ArticleSlug;
