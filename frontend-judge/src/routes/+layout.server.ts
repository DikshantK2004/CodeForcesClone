import type { Actions } from '@sveltejs/kit';
import type {LayoutServerData} from './$types';

export async function load({cookies}){

    let username = cookies.get('username');
    return {
        "username" : username
    }
}
