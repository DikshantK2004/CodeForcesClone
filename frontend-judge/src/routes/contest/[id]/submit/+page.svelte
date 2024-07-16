<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageData } from '../$types';
	import '$lib/contest_styles.css';
	import ErrorDialog from '$lib/error_dialog.svelte';

	export let data: PageData;
	export let form: FormData;
</script>

<div class="flex-in left">
	<h2>Submit</h2>
	<form id="form-group" method="POST" action="?/submit_code" use:enhance>
		<table>
			<tr>
				<td class="label-tag"><label for="problem_name">Problem:</label></td>
				<td class="input-tag">
					<select name="problem_name">
						{#each data.contest.problems as problem}
							<option value={problem.id}>{problem.name}</option>
						{/each}
					</select>
				</td>
			</tr>
			<tr>
				<td class="label-tag"><label for="extension">Language: </label></td>
				<td class="input-tag">
					<select name="extension">
						<option value="py">python</option>
						<option value="c++">C++</option>
					</select>
				</td>
			</tr>
			<tr>
				<td class="label-tag"><label for="code">Code: </label></td>
				<td class="input-tag"><textarea name="code" /></td>
			</tr>
		</table>
		<input type="submit" value="Submit" />
	</form>
</div>

<ErrorDialog htmlContent={form?.error} visible={form?.error !== undefined} message="" />

<style>
	#form-group {
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: start;
		font-family: Arial, sans-serif;
		font-size: 26px;
	}

	#form-group > input {
		align-self: center;
		width: 80px;
		font-size: 20px;
	}
	label {
		margin-right: 20px;
	}

	select {
		font-size: 18px;
		text-align: center;
		min-width: 160px;
		max-width: 80%;
	}

	table {
		width: 80%;
	}
	.label-tag {
		text-align: right;
		max-width: 40px;
	}

	td {
		padding-bottom: 30px;
	}

	.input-tag {
		text-align: left;
		min-width: 80px;
		max-width: 60%;
	}

	textarea {
		width: 85%;
		height: 300px;
		border-radius: 5px;
		font-family: monospace;
		font-size: 14px;
		line-height: 1.5;
		padding: 10px;
		font-size: 18px;
		border: 1px solid black;
		/* Add the following line */
		resize: none;
	}
</style>
