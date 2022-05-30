import {ActionFunction, FormData} from "@remix-run/node";

export const action: ActionFunction = async ({request}) =>{
    let body = await request.formData();
    let slug = body.get("slug");
    let field = body.get("field");
    let token = process.env.TOKEN;
    const headers = {
        headers: {
            Authorization: token!.toString(),
            ContentType: "application/json",
        },
    };
    const res = await fetch(`http://127.0.0.1:5500/article/?slug=${slug}&field=${field}`, headers);
    if (res.statusText === "401") {
        return {
            error: true,
            message: "Problems fetching article from backdoor",
        };
    }
    return "";
}
