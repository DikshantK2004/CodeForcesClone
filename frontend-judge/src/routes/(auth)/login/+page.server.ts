import { fail, type Actions } from "@sveltejs/kit";

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
    login: async ({request}) => {

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
        
        return res.ok? {
            status: 200,
            body: data
        } : {
            status: res.status,
            error: data,
        }
    }
} satisfies Actions;