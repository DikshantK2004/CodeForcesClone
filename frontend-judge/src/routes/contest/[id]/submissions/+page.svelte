<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
	import '$lib/contest_styles.css';
	import SubmissionDetails from '$lib/submission_details.svelte'
	import { error } from '@sveltejs/kit';
	let submissions = data.submissions;


	let details_visible = false;
	let submission_id = 0;
	if (data?.error) {
		error(404, data.error);
	}
</script>

<div class="flex-in left">
	<div class="submissions-section">
		<h2>Submissions</h2>
		<table>
			<thead>
				<tr>
					<th>#</th>
					<th>When</th>
					<th>Who</th>
					<th>Problem</th>
					<th>Lang</th>
					<th>Verdict</th>
					<th>Time</th>
				</tr>
			</thead>
			<tbody>
				{#each submissions as submission}
					<tr>
						<!-- svelte-ignore a11y-click-events-have-key-events -->
						<!-- svelte-ignore a11y-no-static-element-interactions -->
						<!-- svelte-ignore a11y-missing-attribute -->
						<td><a on:click={()=>{details_visible=true; submission_id= submission.id;}}>{submission.id}</a></td>
						<td>{submission.created_at}</td>
						<td>{submission.username}</td>
						<td
							><a href="/contest/{data.contest.id}/problems/{submission.problem_num}"
								>{submission.problem_name}</a
							></td
						>
						<td>{submission.ext}</td>
						<td
							><div
								class:verdict-green={submission.verdict == 'Accepted'}
								class:verdict-red={submission.verdict[0] == 'W'}
							>
								{submission.verdict}
							</div></td
						>
						<td>{submission.time_taken} ms</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>
{#if details_visible}
<SubmissionDetails bind:details_visible bind:submission_id/>
{/if}


<style>
	.left {
		width: 100%;
		padding: 20px;
	}
	table {
		width: 100%;
		border-collapse: collapse;
		margin-top: 1rem;
	}
	th,
	td {
		border: 1px solid #ccc;
		padding: 8px;
		text-align: left;
	}
	thead {
		background-color: #f4f4f4;
	}
	th {
		font-weight: bold;
	}
	tbody tr:nth-child(even) {
		background-color: #fafafa;
	}
	.submissions-section {
		padding: 20px;
		font-family: Arial, sans-serif;
	}

	a {
		text-decoration: none;
		color: #615efc;
	}

	.verdict-green {
		color: green;
		font-weight: bold;
	}
	.verdict-red {
		color: red;
		font-weight: bold;
	}
</style>
