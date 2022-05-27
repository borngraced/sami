import { Link } from "@remix-run/react";
import React from "react";

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
    let rand = Math.floor(Math.random() * emojis.length);
    let show_tags = article?.tags?.map((e: String) => {
      return (
        <span className="tags text-xs text-gray-400">{"#" + e + " "}</span>
      );
    });
    return (
      <div className="key" key={String(article.uuid) + "sami"}>
        <Link to={`/${article.slug}`}>
          <h2 className="transition-all ease-in title font-mono text-xl hover:text-gray-300 cursor-pointer">
            {article.title.toLocaleLowerCase() + " " + emojis[rand]}
          </h2>
          <p className="transition-all ease-in summary font-mono">
            {article.summary}...
          </p>
          <div className="tags inline">{show_tags}</div>
        </Link>
      </div>
    );
  });
  return (
    <div className="block my-4 p-3 md:p-0">
      <div className="font-mono text-xl border-b-2 border-orange-100 py-4">
        .....💭
      </div>
      <div className="articles block my-4">{show_articles}</div>
    </div>
  );
}

export default Articles;
