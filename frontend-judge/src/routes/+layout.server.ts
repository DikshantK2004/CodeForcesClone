export async function load({ cookies }) {
	let username = cookies.get('username');
	return {
		username: username
	};
}
