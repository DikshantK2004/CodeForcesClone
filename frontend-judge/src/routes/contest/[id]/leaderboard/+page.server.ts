import { fail, json } from '@sveltejs/kit';

export const load = async ({ params,fetch }) => {
    const res = await fetch(`http://localhost:8000/submit/leaderboard/${params.id}`);
    if(!res.ok)
        return fail(res.status, {error: 'Failed to fetch leaderboard data'});
    const jsdata = await res.json();

    jsdata.map((element: {
        data: { [problem_num: number]: {}; };cells:[{problem_num: number}]
}) => {
        let cells: { [problem_num: number]: {} } = {};
        for (let cell of element.cells){
            cells[cell.problem_num] = cell;
        }

        element.data = cells;
    
    });

    for(let i=0;i<jsdata.length;i++){
        jsdata[i].rank = i + 1;
        delete jsdata[i].cells;
    }
    return {
        'leaderboard' : jsdata
    }
}