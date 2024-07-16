export const load = async ({ parent, data }) => {
	await parent();

	let { leaderboard } = data;

	for (let userData of leaderboard) {
		for (let cell in userData.data) {
			let submissions = userData.data[cell].submissions;
			let accept: boolean = false;
			for (let submission of submissions) {
				accept = accept || submission.verdict === 'Accepted';
			}
			userData.data[cell].accept = accept;
		}
	}

	return { leaderboard };
};
