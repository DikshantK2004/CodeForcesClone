import { fail } from "@sveltejs/kit";
const istOffset = 5.5 * 60 * 60 * 1000;
export const load = async ({fetch, params} ) =>{
    let res = await fetch('http://localhost:8000/contests/particular/' + params.id);

    if(!res.ok)
        return fail(res.status, {error: 'Failed to fetch the contest'});
    let contest = await res.json();
    console.log(contest);
    let start_mills = new Date(contest.start_date).getTime();
    let end_millis = new Date(contest.end_date).getTime();
    contest.start_millis = start_mills + istOffset;
    contest.end_millis = end_millis + istOffset;

    return { contest};
}