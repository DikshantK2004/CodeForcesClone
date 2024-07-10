<script lang="ts">
	import { error } from '@sveltejs/kit';
	import type { PageData } from './$types';
    import '$lib/contest_styles.css';
	export let data: PageData;
	const problem_wise_stats = data.contest.problem_wise_stats;
	console.log(problem_wise_stats);
	if (data?.error) {
		error(404, data.error);
	}
</script>

<div class="flex-in left">
	<h3 style="margin:20px">Problems</h3>
	<table>
		<thead>
			<tr>
				<th>#</th>
				<th>Problem Name</th>
				<th>Time Limit(in ms)</th>
				<th> Accepted</th>
			</tr>
		</thead>
		<tbody>
			{#each data.contest.problems as problem}
				{#if problem_wise_stats[problem.id]}
					<tr class="colored-row-{problem_wise_stats[problem.id].accepted > 0 ? 'green' : 'red'}">
						<td>{problem.problem_num}</td>
						<td>{problem.name}</td>
						<td>{problem.time_limit}</td>
						<td>{problem.accepted}</td>
					</tr>
				{:else}
					<tr>
						<td>{problem.problem_num}</td>
						<td>{problem.name}</td>
						<td>{problem.time_limit}</td>
						<td>{problem.accepted}</td>
					</tr>
				{/if}
			{/each}
		</tbody>
	</table>
</div>

<style>
	.colored-row-green {
		background-color: #def9c4;
	}

	.colored-row-red {
		background-color: #f9c4c4;
	}

	.left {
		width: 100%;
		padding: 20px;
	}

	.left h3 {
		width: min-content;
	}
	table {
		border-collapse: collapse;
		width: 100%;
	}
	thead {
		background-color: #eeeeee;
	}
	th,
	td {
		padding: 15px;
		border-bottom: 1px solid #ddd;
		text-align: left;
	}

	th:nth-child(2) {
		width: 50%;
	}
</style>
