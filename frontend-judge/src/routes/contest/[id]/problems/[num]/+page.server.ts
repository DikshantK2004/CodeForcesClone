import snarkdown from 'snarkdown';
import {compile} from 'mdsvex';

export const load = async ({params, cookies, fetch}) =>{
    let contestId= params.id;
    let problemNum = params.num;



    let res = await fetch('http://localhost:8000/problems/' + contestId + '/' + problemNum);
    if(!res.ok){
        return {
            'error' : 'An Error Occured'
        }
    }

    let problem = await res.json();
    problem.statement = await compile(problem.statement);
    problem.statement = problem.statement.code;
    return {problem}
}