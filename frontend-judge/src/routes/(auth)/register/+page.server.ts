import { error, fail, redirect, type Actions } from "@sveltejs/kit";
import { readStream } from "$lib/utils";

export const actions ={
    register: async ({request, cookies}) => {

        // console.log('login');
        const body = await request.formData();
        
        const email = body.get('email');
        const password = body.get('password');
        const username = body.get('username');
        const name = body.get('name');
        // console.log(body);

        if(email === '' || password === '' || username === '' || name === '')
            return fail(404, {'error' :'All fields are required'});
        
        if (!email || !password || !username || !name) 
            return fail(404, {'error' :'All fields are required'});
        
        if (password.toString().length < 8)
            return fail(404, {'error' :'Password must be at least 8 characters long'});

        const res = await fetch('http://localhost:8000/register', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({email, password, username, name})
        });
        let resText = res.body;
        
        const data : string = await readStream(res.body!);
        // console.log(res);

        if(! res.ok)
            return fail(res.status, {error: data});
        
        return redirect(302, '/login');
    }
} satisfies Actions;