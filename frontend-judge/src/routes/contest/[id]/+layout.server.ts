import { read } from '$app/server';
import { readStream } from '$lib/utils.js';

const istOffset = 5.5 * 60 * 60 * 1000;

export const load = async ({ fetch, params, cookies }) => {
	const username = cookies.get('username') ?? '1';
	const contestResponse = await fetch(`http://localhost:8000/contests/particular/${params.id}`);
	if (!contestResponse.ok) {
		return {
			status: contestResponse.status,
			error: 'Failed to fetch contest'
		};
	}

	const contest = await contestResponse.json();
	const userContestResponse = await fetch(
		`http://localhost:8000/submit/user_contest/${username}/${params.id}`
	);

	// const startMillis = new Date(contest.start_date).getTime() + istOffset;
	// const endMillis = new Date(contest.end_date).getTime() + istOffset;
	let dat = userContestResponse.body!;
	if (!userContestResponse.ok) return { contest };

	const submissions = await userContestResponse.json();
	let problemWiseStats: { [problem_id: number]: { accepted: number; total: number } } = {};

	submissions.forEach((submission: { problem_num: number; verdict: string }) => {
		const { problem_num, verdict } = submission;
		if (!problemWiseStats[problem_num]) {
			problemWiseStats[problem_num] = { accepted: 0, total: 0 };
		}
		if (verdict === 'Accepted') {
			problemWiseStats[problem_num].accepted++;
		}
		problemWiseStats[problem_num].total++;
	});

	contest.problem_wise_stats = problemWiseStats;
	return { contest };
};
