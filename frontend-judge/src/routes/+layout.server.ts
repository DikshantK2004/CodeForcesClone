import type {LayoutServerData} from './$types';

export async function load({request}){

    let username = request.headers.getSetCookie();
    console.log(username);
    return {
        status: 200,
        headers: {
            'content-type': 'application/json'
        },
        body: JSON.stringify({})
    };
}