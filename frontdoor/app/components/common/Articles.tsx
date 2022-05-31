import { Link } from "@remix-run/react";
import React from "react";
import Tags from "~/components/common/Tags";

export interface ArticleIT {
  uuid: Number;
  slug: String;
  title: String;
  content: String;
  summary: String;
  tags: String[];
  likes: Number;
  created_at: Date;
  updated_at: Date;
}

type Props = {
  articles: ArticleIT[];
};

let emojis = ["😇", "🤪", "🥳", "🤩", "🤯", "🤗", "❤️", "💙", "💚", "💜"];
function Articles({ articles }: Props) {
  const show_articles = articles?.map((article: ArticleIT) => {
    let emojisRand = Math.floor(Math.random() * emojis.length);

    return (
      <div
        className="transition-all ease-in delay-75 bg-slate-800 shadow-md p-4 font-semibold rounded-md text-white text-opacity-70 hover:scale-105"
        key={String(article.uuid) + "sami"}
      >
        <Link to={`/${article.slug}`}>
          <h2 className="transition-all ease-in delay-100 title font-mono text-md md:text-xl hover:text-gray-300 cursor-pointer tracking-wider pb-1">
            {article.title.toLocaleLowerCase() + " " + emojis[emojisRand]}
          </h2>
          <p className="transition-all ease-in delay-150 summary text-sm md:text-md font-mono font-light italic tracking-wide pb-3">
            {article.summary}...
          </p>
          <Tags tags={article?.tags} />
        </Link>
      </div>
    );
  });
  return (
    <div className="block my-8 p-3 md:p-0">
      <div className="font-mono text-white text-opacity-70 text-xl md:text-5xl pt-4 tracking-widest">
        ... 💭
      </div>
      <div className="articles flex flex-col my-4 gap-y-4">{show_articles}</div>
    </div>
  );
}

export default Articles;
