<script lang="ts">
    import type {PageData} from "./$types";
    import '$lib/contest_styles.css';
    export let data: PageData;
</script>
<div class="flex-container">
    <slot/>
	<div class="flex-in right">
		<div style="font-weight:bold; font-size:24px;padding-bottom:0px; ">
			<a href="/contest/{data.contest.id}" data-sveltekit-reload>{data.contest.name}</a><br />
			<div style="font-size:20px; color:#615EFC;padding-bottom:0px;">
				{#if Date.now() < data.contest.end_msecs}
					Contest is live
				{:else}
					Finished
				{/if}
			</div>
		</div>
            <div style="height:25px; background-color:#EfEDEB80; width:100%; padding:0;"></div>
            <a href="/contest/{data.contest.id}" data-sveltekit-reload>Problems</a>
            <a href="/contest/{data.contest.id}/submissions" data-sveltekit-reload>Submissions</a>
            <a href="/contest/{data.contest.id}/submissions/my" data-sveltekit-reload>My Submissions</a>
            <a href="/" data-sveltekit-reload>Leaderboard</a>
            <a href="/contest/{data.contest.id}/submit" data-sveltekit-reload>Submit Code</a>
        
	</div>

    
</div>
