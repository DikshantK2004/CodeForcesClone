<script lang="ts">
	import { onMount } from 'svelte';
	import type { sub_type } from '$lib/utils';
	import { convertToLocalIST } from '$lib/utils';
	import SubmissionDetails from '$lib/submission_details.svelte';
	import Prism from 'prismjs';
	
	export let visible = false;
	export let submission_data: sub_type;

	let details_visible = false;
	let submission_id = 0;
	let dialog: HTMLDialogElement;
	let closeDialog = () => {
		visible = false;
	};

	$: if (dialog && visible) {
		dialog.showModal();
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->

<dialog
	bind:this={dialog}
	on:close={() => {
		visible = false;
	}}
	on:click|self={() => dialog.close()}
>
	<button
		class="close-button"
		on:click|stopPropagation={() => {
			dialog.close();
			submission_data = undefined;
		}}>⨯</button
	>
	<div class="content">
		{#if submission_data}
			{#each submission_data as submission}
				<div class="row">
					{convertToLocalIST(submission.created_at)}
					&nbsp;
					<span
						class:verdict-green={submission.verdict === 'Accepted'}
						class:verdict-red={submission.verdict.startsWith('Wrong')}
						class:verdict-others={!submission.verdict.startsWith('Accepted') &&
							!submission.verdict.startsWith('Wrong')}>{submission.verdict}</span
					>
					&nbsp; ⇒<a on:click={()=>{submission_id=submission.submission_id; details_visible = true; dialog.close();}}> {submission.submission_id}</a>
				</div>
			{/each}
		{/if}
		
	</div>
	
</dialog>
{#if details_visible}
<SubmissionDetails bind:details_visible bind:submission_id/>
{/if}
<style>

	.row {
		margin: 3px;
	}
	dialog {
		background-color: white;
		border-radius: 8px;
		border: 4px solid #00000080;
		box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.2);
		padding: 16px;
		font-family: Arial, Helvetica, sans-serif;
		font-size: 22px;
	}

	button.close-button {
		position: absolute;
		top: 8px;
		right: 8px;
		background-color: #d2e2d2;
		border: none;
		cursor: pointer;
	}

	

	.content {
		border-radius: 8px;
		width: 55vw;
		text-align: left;
		height: 55vh;
		margin: 16px;
		padding: 16px;
		border: 3px solid #ddd;
	}

	.verdict-green {
		color: green;
	}
	.verdict-red {
		color: red;
	}

	.verdict-others {
		color: #2e236c;
	}

	a {
		color: #615efc;
	}
</style>
