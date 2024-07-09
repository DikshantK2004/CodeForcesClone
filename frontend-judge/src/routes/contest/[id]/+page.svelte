<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div class="flex-container">
	<div class="flex-in left">
	<h3 style="margin:20px"> Problems</h3>
     <table>
        <thead>
            <tr>
                <th>#</th>
                <th>Problem Name</th>
                <th>Time Limit(in ms)</th>
            </tr>
        </thead>
        <tbody>
            {#each data.contest.problems as problem}
                <tr>
                    <td>{problem.problem_num}</td>
                    <td>{problem.name}</td>
                    <td>{problem.time_limit}</td>
                </tr>
            {/each}
        </tbody>
     </table>
    </div>
	<div class="flex-in right">
		<div style="font-weight:bold; font-size:24px;padding-bottom:0px; ">
			<a href="/contest/{data.contest.id}">{data.contest.name}</a><br />
			<div style="font-size:20px; color:#615EFC;padding-bottom:0px;">
				{#if Date.now() < data.contest.end_msecs}
					Contest is live
				{:else}
					Finished
				{/if}
			</div>
		</div>
            <div style="height:25px; background-color:#EfEDEB80; width:100%; padding:0;"></div>
            <a href="/contest/{data.contest.id}">Problems</a>
            <a href="/">Submissions</a>
            <a href="/">Leaderboard</a>
            <a href="/">Submit Code</a>
        
	</div>
</div>

<style>
	.flex-container {
		display: flex;
		padding: 40px;
		column-gap: 30px;
		justify-content: space-between;
		font-size: 18px;
		text-align: center;
	}

	.flex-in {
		background-color: #f2f2f2;
		box-shadow: 4px 4px 0 #69666640;
		border-radius: 20px;
	}

	.right {
		width: 25%;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.right div {
		padding: 10px;
	}

    .right a{
        padding: 5px;
    }


    a{
        text-decoration: none;
        color: black;
    }

    .left{
        width: 100%;
        padding: 20px;
    }

    .left h3{
        width:min-content;
    }
    table {
		border-collapse: collapse;
        width: 100%;
	}
    thead{
        background-color: #EEEEEE;
    }
	th,td {
		padding: 15px;
		border-bottom: 1px solid #ddd;
        text-align:left;
	}

    th:nth-child(2){
        width: 50%;;
    }

</style>
