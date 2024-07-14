<script lang="ts">
    import "$lib/contest_styles.css";
    import type {PageData} from "./$types";
    import type{sub_type} from "$lib/utils";
    import SubmissionDialog from './submissions_dialog.svelte';
    export let data: PageData;
    let visible = false;
    let leaderboard = data.leaderboard;
    let num_problems:number = data.contest.problems.length;
    let submission_data : sub_type;
    function showDialog(submissions : sub_type){
        submission_data = submissions;
        visible = true;
    }
</script>

<div class="left flex-in">
<h2>Contest Leaderboard</h2>
    <table>
        <thead>
            <tr>
                <th>#</th>
                <th>Username</th>
                {#each Array.from({ length: num_problems }, (_, i) => i + 1) as problemNum}
                    <th class="prob"><a href="/contest/{data.contest.id}/problems/{problemNum}">{problemNum}</a></th>
                {/each}
                 
            </tr>
        </thead>
        <tbody>
            {#each leaderboard as entry}
                <tr>
                    <td style="width:40px;">{entry.rank}</td>
                    <td style="max-width:30%;">{entry.username}</td>
                    {#each Array.from({ length: num_problems }, (_, i) => i + 1) as problemNum}
                        {#if entry.data[problemNum] }
                            <td class="prob" class:verdict-green={entry.data[problemNum].accept} class:verdict-red={!entry.data[problemNum].accept}>
                                <!-- svelte-ignore a11y-click-events-have-key-events -->
                                <!-- svelte-ignore a11y-no-static-element-interactions -->
                                <span on:click={()=>{showDialog(entry.data[problemNum].submissions);}}>{entry.data[problemNum].accept?'+' :'-'}{entry.data[problemNum].submissions.length}</span>
                            </td>
                        {:else}
                            <td class="prob"></td>
                        {/if}
                    {/each}
                </tr>
            {/each}
        </tbody>


    </table>

</div>
<SubmissionDialog bind:visible bind:submission_data></SubmissionDialog>

<style>
     table {
        width: 100%;
        border-collapse: collapse;
        margin-top: 1rem;
    }
    th, td {
        /* border: 1px solid #ccc; */
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
    

    a{
        text-decoration: none;
        color: #615EFC;
    }

    .verdict-green{
        color: green;
    }
    .verdict-red{
        color:red;
    }

    thead{
        background-color: #d2d2e2;
    }
    .prob{
        text-align: center;
        font-size:18px;
    }
</style>