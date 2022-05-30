import { Link } from "@remix-run/react";
import React from "react";
import Layout from "~/components/layout";

type Props = {};

const Header = (props: Props) => {
  return (
   <>
       <Layout>
           <Link to="../" className="cursor-pointer">
               <div className="flex justify-between items-center w-full h-full text-left p-2 text-black font-mono pb-5">
                   <h2 className="text-2xl md:text-4xl font-medium antialiased text-white text-opacity-80 tracking-widest">sami</h2>
                   <span className="date text-xs font-light opacity-40 text-white text-opacity-80">
          {"⏳" + " " + new Date().toLocaleDateString()}
        </span>
               </div>
           </Link>
       </Layout>
       <div className="h-1 w-full border-b-2 border-slate-800 shadow-md"></div>
   </>
  );
};

export default Header;
