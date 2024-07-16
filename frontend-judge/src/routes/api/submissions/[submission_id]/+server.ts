import { error, json } from "@sveltejs/kit";

export async function GET({params}){
    let submission_id = params.submission_id;
    let res = await fetch('http://localhost:8000/submit/particular/' + submission_id);
    if (!res.ok) {
        return error(res.status, res.statusText);
    }
    let submission = await res.json();
    return json( submission );
}