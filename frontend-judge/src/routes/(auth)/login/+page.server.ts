import { error, fail, type Actions } from "@sveltejs/kit";
import { readStream } from "$lib/utils";

export const actions ={
    login: async ({request, cookies}) => {

        console.log('login');
        const body = await request.formData();
        
        const email = body.get('email');
        const password = body.get('password');
        console.log(body);
        
        if (!email || !password) {
            return {
                status: 400,
                error: 'Email and password are required'
            };
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
        console.log(res);

        cookies.set('AuthorizationToken', `Bearer ${data}`, {
            httpOnly: true,
            path: '/',
            secure: true,
            sameSite: 'strict',
            maxAge: 60 * 60 * 24 // 1 day
          });

       
        return res.ok? {
            status: 200,
        } : {
            status: res.status,
            error: data,
        }
    }
} satisfies Actions;