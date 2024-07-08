import { error, fail, redirect, type Actions } from "@sveltejs/kit";
import { readStream } from "$lib/utils";

export const actions ={
    default: async ({request, cookies}) => {

        // console.log('login');
        const body = await request.formData();
        
        const email = body.get('email');
        const password = body.get('password');
        // console.log(body);
        if(email === '' || password === '' )
            return fail(404, {'error' :'All fields are required'});
        if (!email || !password) {
            return fail(404, {'error' :'Email and password are required'});
        }

        const res = await fetch('http://localhost:8000/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({email, password})
        });
        let resText = res.body;
        
        const data : string = await readStream(res.body!);
        // console.log(res);
        if(! res.ok)
            return fail(res.status, {error: data});
        console.log(cookies);
        
        return redirect(302, '/');
    }
} satisfies Actions;