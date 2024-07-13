import { error, fail, redirect, type Actions } from "@sveltejs/kit";



export const actions = {
    submit_code: async ({request, params, cookies}) => {
        let form_data = await request.formData();

        let code = form_data.get('code');
        let extension = form_data.get('extension');
        let pid = form_data.get('problem_name');
        let problem_id: number;
        try {
            problem_id = Number(pid);
        } catch  {
            return fail(400, {"error": "Problem must be a valid problem"})
        }

        if (!code || !extension || !problem_id)
            return fail(400, {'error': 'All the fields must be present'});

        let token = cookies.get('token');
        if(!token)
            return fail(404, {"error" : "Please login first"});

        let res = await fetch('http://localhost:8000/submit/', {
            method: "POST",
            body: JSON.stringify({code, extension, problem_id}),
            headers: {
                'Content-Type': 'application/json',
                'Cookie': `token=${token}`
            }
        });

        if (!res.ok) {
            return fail(res.status, {"error": "couldn't submit the code"});
        }

        return redirect(302, `/contest/${params.id}/submissions/my`);
    }
} satisfies Actions;