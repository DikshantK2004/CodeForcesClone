import { error, json } from "@sveltejs/kit";

export async function POST({ cookies }) {
    // Remove cookies
    cookies.delete('username', { path: '/' });
    cookies.delete('token', {path:'/'});

    return json({});
}

