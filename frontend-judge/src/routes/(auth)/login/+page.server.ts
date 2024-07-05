import { error, fail, type Actions } from "@sveltejs/kit";

const readStream = async (stream: ReadableStream<Uint8Array>) => {
    const reader = stream.getReader();
    let chunks = '';

    while (true) {
        const { value, done } = await reader.read();
        if (done) {
            break;
        }
        chunks += new TextDecoder().decode(value);
    }
    return chunks;
}
export const actions ={
    login: async ({request, cookies}) => {

        const body = await request.formData();
        
        const email = body.get('email');
        const password = body.get('password');
        
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