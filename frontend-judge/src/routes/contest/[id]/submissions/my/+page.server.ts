
const istOffset = 5.5 * 60 * 60 * 1000;
export const load = async ({params, fetch,cookies}) =>{
    let contestId= params.id;
    let username = cookies.get('username') ?? '1';
    let res = await fetch(`http://localhost:8000/submit/user_contest/${username}/${params.id}` );

    if(!res.ok){
        return {
            'error' : 'An Error Occured'
        }
    }
    let submissions = await res.json();
    submissions.map((submission: { created_at: string | number | Date; })=>{
        submission.created_at = new Date(submission.created_at).getTime() + istOffset;
        submission.created_at = new Date(submission.created_at).toLocaleString();
        
    });
    return {submissions}
}