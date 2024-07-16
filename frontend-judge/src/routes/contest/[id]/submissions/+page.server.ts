const istOffset = 5.5 * 60 * 60 * 1000;
export const load = async ({ params, fetch }) => {
	let contestId = params.id;
	let res = await fetch('http://localhost:8000/submit/contest/' + contestId);

	if (!res.ok) {
		return {
			error: 'An Error Occured'
		};
	}
	let submissions = await res.json();
	submissions.map((submission: { created_at: string | number | Date }) => {
		submission.created_at = new Date(submission.created_at).getTime() + istOffset;
		submission.created_at = new Date(submission.created_at).toLocaleString();
	});
	return { submissions };
};
