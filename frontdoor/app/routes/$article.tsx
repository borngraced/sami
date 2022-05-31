import { json, LoaderFunction } from "@remix-run/node";
import { useLoaderData, useSubmit } from "@remix-run/react";
import React from "react";
import { ArticleIT } from "~/components/common/Articles";
import Tags from "~/components/common/Tags";

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
  const submit = useSubmit();

  const updateLikes = (event: any) => {
    console.log("of");
    submit(event.currentTarget, { replace: true });
  };

  return (
    <main className="mx-auto max-w-4xl my-10 px-4 md:px-0">
      <div className="text-center antialiased font-mono text-white text-opacity-70 mb-6 ">
        <h1 className="text-black text-xl md:text-2xl tracking-widest text-inherit pb-2">
          {article?.data?.title}
        </h1>
        <span className="title text-xs text-inherit">
          date published: {article?.data?.created_at}
        </span>
      </div>
      <div
        className="post-body text-md md:lg py-8 text-white text-opacity-70 antialiased font-mono"
        dangerouslySetInnerHTML={{ __html: article.data.content }}
      />

      <Tags tags={article?.data?.tags} />

      <div className="block text-sm pt-3">
        <form action="/article/messenger" method="put" className="post">
          <input
            type="text"
            name="slug"
            defaultValue={article?.data?.slug}
            hidden
          />
          <input type="text" name="field" defaultValue={"likes"} hidden />
          <input
            type="text"
            name="content"
            defaultValue={article?.data?.likes + 1}
            hidden
          />
          <input type="text" name="method" defaultValue="PUT" hidden />
          <button
            type="submit"
            className="like_post cursor-pointer text-white text-opacity-70 antialiased font-mono"
          >
            Likes 👍 {article?.data.likes}
          </button>
        </form>
      </div>
    </main>
  );
};
export default ArticleSlug;
