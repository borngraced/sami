import React from "react";

interface LayoutIt {
    tags: String[];
}

let borderColors = ["border-green-500", "border-red-500", "border-emerald-500", "border-blue-500", "border-orange-500", "border-purple-500", "border-black"]

const Tags = ({ tags}:LayoutIt) => {
    let bcRand = Math.floor(Math.random() * borderColors.length);
    let show_tags = tags?.map((e: String, idx:number) => {
        return (
            <span className={`tags text-xs text-gray-400 border ${borderColors[bcRand]} shadow-md rounded-md p-1`} key={idx}>{"#" + e + " "}</span>
        );
    });
    
    return (
        <div className="tags flex gap-2">{show_tags}</div>

    )
}

export default Tags
