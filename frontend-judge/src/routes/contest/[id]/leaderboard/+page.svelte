<script lang="ts">
    import "$lib/contest_styles.css";
    import type {PageData} from "./$types";
    export let data: PageData;
    let leaderboard = data.leaderboard;
    let num_problems:number = data.contest.problems.length;
    console.log(leaderboard[0].data[2]);
</script>

<div class="left flex-in">
<h2>Contest Leaderboard</h2>
    <table>
        <thead>
            <tr>
                <th>#</th>
                <th>Username</th>
                {#each Array.from({ length: num_problems }, (_, i) => i + 1) as problemNum}
                    <th><a href="/contest/{data.contest.id}/problems/{problemNum}">{problemNum}</a></th>
                {/each}
                 
            </tr>
        </thead>
        <tbody>
            {#each leaderboard as entry}
                <tr>
                    <td style="width:40px;">{entry.rank}</td>
                    <td style="max-width:30%;">{entry.username}</td>
                    {#each Array.from({ length: num_problems }, (_, i) => i + 1) as problemNum}
                        {#if entry.data[problemNum]}
                            <td class:verdict-green={entry.data[problemNum].accept} class:verdict-red={!entry.data[problemNum].accept}>{entry.data[problemNum].submissions.length}</td>
                        {:else}
                            <td></td>
                        {/if}
                    {/each}
                </tr>
            {/each}
        </tbody>
    </table>
</div>

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
        font-weight: bold;;
    }
    .verdict-red{
        color:red;
        font-weight:bold;
    }

    thead{
        background-color: #d2d2e2;
    }

</style>