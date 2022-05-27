import { Link } from "@remix-run/react";
import React from "react";

type Props = {};

const Header = (props: Props) => {
  return (
    <Link to="../" className="cursor-pointer">
      <div className="block bg-blue-200 w-full h-full text-center p-2 text-black font-mono ">
        <h2 className="text-xl font-medium antialiased">borngraced(sami)</h2>
        <span className="date text-md font-light opacity-40 pt-4">
          {"⏳" + " " + new Date().toLocaleDateString()}
        </span>
      </div>
    </Link>
  );
};

export default Header;
