<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;

	// onDestroy(() => {
	//     clearInterval(interval);
	// });
</script>

<div class="container">
	<h3 style="margin:20px;">Upcoming Contests</h3>

	<table>
		<thead>
			<tr>
				<th>Contest Name</th>
				<th>Start Time(IST)</th>
				<th> Duration</th>
				<th> Status</th>
			</tr>
		</thead>
		<tbody>
			{#each data.contests as contest}
				{#if contest.end_msecs - Date.now() > 0}
					<tr>
						<td class="name">{contest.name}</td>
						<td class="time">
							<div>{contest.start_date}</div>
							<div>{contest.start_time}</div>
						</td>
						<td>{contest.duration}</td>
						<td class="status">
							{#if contest.start_msecs - Date.now() > 0}
								Not Started
							{:else if contest.start_msecs - Date.now() <= 0 && contest.end_msecs - Date.now() > 0}
								<a href="/contest/{contest.id}">Join</a>
							{/if}
						</td>
					</tr>
				{/if}
			{/each}
		</tbody>
	</table>

	<h3 style="margin:20px;">Past Contests</h3>
	<table>
		<thead>
			<tr>
				<th>Contest Name</th>
				<th>Start Time(IST)</th>
				<th> Duration</th>
				<th> Status</th>
			</tr>
		</thead>
		<tbody>
			{#each data.contests as contest}
				{#if contest.end_msecs - Date.now() <= 0}
					<tr>
						<td class="name">{contest.name}</td>
						<td class="time">
							<div>{contest.start_date}</div>
							<div>{contest.start_time}</div>
						</td>
						<td>{contest.duration}</td>
						<td class="status">
							<a href="/contest/{contest.id}"> Enter </a>
						</td>
					</tr>
				{/if}
			{/each}
		</tbody>
	</table>
</div>

<style>
	.container {
		margin: auto;
		width: auto;
		margin-top: 20px;
		max-width: max-content;
		font-size: 18px;
		background-color: #f2f2f2;
		padding: 1px;
		border-radius: 10px;
        box-shadow: 4px 4px 0 #0b0b0b40;
	}

	table {
		border-collapse: collapse;
		margin: 20px;
	}
	th,td {
		padding: 15px;
		text-align: center;
		border-bottom: 1px solid #ddd;
        width: 125px;
	}

	th {
		background-color: #eeedeb;
	}

	tr:nth-child(odd) {
		background-color: #def9c4;
	}

	tr:nth-child(even) {
		background-color: #9cdba690;
	}

	tr:hover {
		background-color: #ddd;
	}

	.name {
		width: 250px;
	}
</style>
