import { fail } from '@sveltejs/kit';

const istOffset = 5.5 * 60 * 60 * 1000; // IST offset in milliseconds

export const load = async ({fetch} ) =>{
    let res = await fetch('http://localhost:8000/contests/all');

    if(!res.ok)
        return fail(res.status, {error: 'Failed to fetch contests'});
    let data = await res.json();


    data.map((contest : any) => {
        let start_mills = new Date(contest.start_date).getTime();
        let end_millis = new Date(contest.end_date).getTime();

        let start_date = new Date(start_mills + istOffset);
        contest.start_date = start_date.toDateString();
        contest.start_time = start_date.toLocaleTimeString();
        contest.duration = ((end_millis - start_mills)/(1000*60*60)).toFixed(2);
        contest.start_msecs = start_mills;
        contest.end_msecs = end_millis;
        return contest;
    });

    return {
        contests: data
    }
}